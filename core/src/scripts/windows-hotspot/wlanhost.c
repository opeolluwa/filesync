#ifdef UNICODE
#undef UNICODE
#endif

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif

#include <Windows.h>
#include <wtypes.h>							//Wireless Networking types
#include <Wlanapi.h>						//Wireless Networking API
#include <stdlib.h>
#include <stdio.h>

//#include <inc\krios.h>
#include "wlanhost.h"


#pragma comment(lib, "wlanapi.lib")			//Wireless Networking API libary
WCHAR *adapter_id;
HANDLE wlanHandle = 0;

void __stdcall wlan_callback(PWLAN_NOTIFICATION_DATA pNotifData, PVOID pContext){

	if (pNotifData->NotificationCode == wlan_hosted_network_state_change)
	{

		PWLAN_HOSTED_NETWORK_STATE_CHANGE pStateChange = (PWLAN_HOSTED_NETWORK_STATE_CHANGE)(pNotifData->pData);
		if (pStateChange->NewState == wlan_hosted_network_idle || pStateChange->NewState == wlan_hosted_network_unavailable)
		{
			printf("err\n");
		}
	}
	else if (pNotifData->NotificationCode == wlan_hosted_network_peer_state_change)
	{
		/* A device has joined or left the network */

		PWLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE pPeerStateChange = (PWLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE)(pNotifData->pData);
		if (wlan_hosted_network_peer_state_authenticated == pPeerStateChange->NewState.PeerAuthState)
		{
			// new connection
			printf("Connection => \n");
			for (int i = 0; i < (int)6; i++) {

				if (i == (6 - 1))
					printf("%.2X\n", (int)pPeerStateChange->NewState.PeerMacAddress[i]);
				else
					printf("%.2X-", (int)pPeerStateChange->NewState.PeerMacAddress[i]);
			}
		}
		else if (wlan_hosted_network_peer_state_invalid == pPeerStateChange->NewState.PeerAuthState)
		{
			// closed connection
		}
	}
}

void __stdcall init_wlan() {
	DWORD pdwNegotiatedVersion;
	WlanOpenHandle(WLAN_API_VERSION_2_0, NULL, &pdwNegotiatedVersion, &wlanHandle);

	WlanHostedNetworkInitSettings(wlanHandle, NULL, NULL);

	WlanRegisterNotification(wlanHandle,
		WLAN_NOTIFICATION_SOURCE_HNWK,
		TRUE, &wlan_callback,
		NULL, NULL, NULL);
}

void __stdcall relase_wlan() {
	if (wlanHandle != NULL) {
		WlanCloseHandle(wlanHandle, NULL);
		wlanHandle = NULL;
		free(adapter_id);
	}
}

DWORD updateProperties() {
	// status update
}

DWORD wlan_updatestatus() {
	PWLAN_HOSTED_NETWORK_STATUS status = NULL;
	auto ret =
		WlanHostedNetworkQueryStatus(wlanHandle, &status, NULL);
	if (ret == ERROR_SUCCESS) {

		adapter_id = (WCHAR*)malloc(100 * sizeof(WCHAR));
		swprintf(
			adapter_id,
			100,
			L"{%08lX-%04hX-%04hX-%02hhX%02hhX-%02hhX%02hhX%02hhX%02hhX%02hhX%02hhX}",
			status->IPDeviceID.Data1, status->IPDeviceID.Data2,
			status->IPDeviceID.Data3, status->IPDeviceID.Data4[0],
			status->IPDeviceID.Data4[1], status->IPDeviceID.Data4[2],
			status->IPDeviceID.Data4[3], status->IPDeviceID.Data4[4],
			status->IPDeviceID.Data4[5], status->IPDeviceID.Data4[6],
			status->IPDeviceID.Data4[7]);
		//MessageBoxW(nul, adapter_id, L"", MB_OK);
		WlanFreeMemory(status);
	}
	return ret;
}

WLAN_HOSTED_NETWORK_REASON set_properties(char* ssid, char* pwd, DWORD maxPeers) {

	WLAN_HOSTED_NETWORK_REASON reason = { 0 };
	WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS settings = { 0 };

	settings.dwMaxNumberOfPeers = maxPeers;
	size_t ssidLen = strlen(ssid);
	settings.hostedNetworkSSID.uSSIDLength = ssidLen;
	memcpy(settings.hostedNetworkSSID.ucSSID, (void*)ssid, ssidLen);

	auto ret = WlanHostedNetworkSetProperty(
		wlanHandle,
		wlan_hosted_network_opcode_connection_settings,
		sizeof(WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS),
		&settings,
		&reason,
		NULL);

	if (ret != ERROR_SUCCESS || reason != wlan_hosted_network_reason_success)
		return reason;

	size_t keyLength = strlen(pwd) + 1;

	ret = WlanHostedNetworkSetSecondaryKey(wlanHandle, keyLength, (PUCHAR)pwd, TRUE, TRUE, &reason, NULL);

	if (ret == ERROR_SUCCESS && reason == wlan_hosted_network_reason_success) {

		//updateProperties;
	}

	return reason;
}

WLAN_HOSTED_NETWORK_REASON turn_on() {
	WLAN_HOSTED_NETWORK_REASON reason;
	WlanHostedNetworkForceStart(wlanHandle, &reason, NULL);
	WlanRegisterVirtualStationNotification(wlanHandle, TRUE, NULL);
	return reason;
}

WLAN_HOSTED_NETWORK_REASON turn_off() {
	WLAN_HOSTED_NETWORK_REASON reason;
	WlanHostedNetworkForceStop(wlanHandle, &reason, NULL);
	return reason;
}

WLAN_HOSTED_NETWORK_REASON set_enabled(BOOL enable) {
	WLAN_HOSTED_NETWORK_REASON reason;
	WlanHostedNetworkSetProperty(wlanHandle,
		wlan_hosted_network_opcode_enable, sizeof(BOOL),
		&enable, &reason, NULL);

	return reason;
}

void __stdcall start_wifi_hotspot(char* ssid, char* pwd) {
	init_wlan();
	auto ret = set_enabled(TRUE);
	if (ret != wlan_hosted_network_reason_success)
	{
		return;  // get out !
	}

	ret = set_properties(ssid, pwd, 10);

	if (ret != wlan_hosted_network_reason_success)
	{
		return;  // get out !
	}

	turn_on();

	wlan_updatestatus();
}


void __stdcall stop_wifi_hotspot() {
	turn_off();
	relase_wlan();
}
