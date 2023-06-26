#include <stdio.h>
#include <stdlib.h>
#include <NetCon.h>
#include <SensAPI.h>

#include "icservice.h"

INetSharingManager *sharing_manager = NULL;

HRESULT init_icservice() {
	HRESULT hr = -1;

	CoInitialize(NULL);
	CoInitializeSecurity(NULL, -1, NULL, NULL,
		RPC_C_AUTHN_LEVEL_PKT,
		RPC_C_IMP_LEVEL_IMPERSONATE,
		NULL, EOAC_NONE, NULL);
	hr = CoCreateInstance(
		&(CLSID_NetSharingManager),
		NULL,
		CLSCTX_ALL,
		&(IID_INetSharingManager),
		(void**)&sharing_manager);

	return hr;
}


void __stdcall release_icservice() {
	sharing_manager->lpVtbl->Release(sharing_manager);
	sharing_manager = NULL;
}

#define ICS_Error_FailGetEvery -32
#define ICS_Error_FailGetNewEnum -33
#define ICS_Error_FailGetEnumVariant -34
#define ICS_Error_EnableSharing -35

HRESULT share_work(INetSharingManager * pNSM, WCHAR* virtual_adapter)
{

	INetSharingEveryConnectionCollection * pNSECC = NULL;
	HRESULT hr = pNSM->lpVtbl->get_EnumEveryConnection(pNSM, &pNSECC);
	int LastErrorCode = 0;
	if (!pNSECC)
		return ICS_Error_FailGetEvery;
	else {

		// enumerate connections
		IEnumVARIANT * pEV = NULL;
		IUnknown * pUnk = NULL;
		hr = pNSECC->lpVtbl->get__NewEnum(pNSECC, &pUnk);
		if (pUnk) {
			hr = pUnk->lpVtbl->QueryInterface(pUnk, &(IID_IEnumVARIANT),
				(void**)&pEV);
			pUnk->lpVtbl->Release(pUnk);
		}
		else {
			return ICS_Error_FailGetNewEnum;
		}
		if (pEV) {
			VARIANT v;
			VariantInit(&v);
			while (S_OK == pEV->lpVtbl->Next(pEV, 1, &v, NULL)) {
				if (V_VT(&v) == VT_UNKNOWN) {
					INetConnection * pNC = NULL;
					V_UNKNOWN(&v)->lpVtbl->QueryInterface
						(V_UNKNOWN(&v), &(IID_INetConnection),
						(void**)&pNC);
					if (pNC) {
						INetConnectionProps * pNCP = NULL;
						pNSM->lpVtbl->get_NetConnectionProps(pNSM, pNC, &pNCP);
						if (pNCP) {
							// check properties for firewalled or shared connection
							NETCON_MEDIATYPE MediaType;
							pNCP->lpVtbl->get_MediaType(pNCP, &MediaType);
							NETCON_STATUS Status;
							pNCP->lpVtbl->get_Status(pNCP, &Status);
							BSTR DevName;
							pNCP->lpVtbl->get_DeviceName(pNCP, &DevName);
							BSTR devGuid;
							pNCP->lpVtbl->get_Guid(pNCP, &devGuid);

							if (MediaType & (NCM_LAN | NCM_SHAREDACCESSHOST_LAN | NCM_PHONE)) {

								/*wcscmp(devGuid, L"{882F0857-8811-4664-88D9-1C1AA08F69AE}") == 0 && */
								if (Status == NCS_CONNECTED && wcscmp(devGuid, virtual_adapter) != 0
									&& wcsstr(DevName, L"Virtual") == 0
									) {

									INetSharingConfiguration * pNSC = NULL;
									hr = pNSM->lpVtbl->get_INetSharingConfigurationForINetConnection(pNSM, pNC, &pNSC);

									if (pNSC) {
										hr = pNSC->lpVtbl->EnableSharing(pNSC, ICSSHARINGTYPE_PUBLIC);
										if (hr != S_OK)
											LastErrorCode = ICS_Error_EnableSharing;
									}
									pNSC->lpVtbl->Release(pNSC);
									pNSC = NULL;
								}

								if (wcscmp(devGuid, /*L"{2682DD96-0DB3-4F6D-9230-F36BC70697E5}"*/ virtual_adapter) == 0) {

									INetSharingConfiguration * pVWifiNSC = NULL;
									hr = pNSM->lpVtbl->get_INetSharingConfigurationForINetConnection(pNSM, pNC, &pVWifiNSC);

									if (pVWifiNSC) {
										hr = pVWifiNSC->lpVtbl->EnableSharing(pVWifiNSC, ICSSHARINGTYPE_PRIVATE);
										if (hr != S_OK) {
											LastErrorCode = ICS_Error_EnableSharing;
										}
									}
									pVWifiNSC->lpVtbl->Release(pVWifiNSC);
									pVWifiNSC = NULL;
								}

							}
							pNCP->lpVtbl->Release(pNCP);
						}

						pNC->lpVtbl->Release(pNC);
					} // pnc
				}
				VariantClear(&v);
			}
			pEV->lpVtbl->Release(pEV);
		}
		else {
			return ICS_Error_FailGetEnumVariant;
		}
		pNSECC->lpVtbl->Release(pNSECC);
	}
	if (LastErrorCode != 0) return LastErrorCode;
	return hr;
}

void __stdcall share_connection(
	__in WCHAR* virtual_adapter
	) 
{
	/* share wifi connection */

	HRESULT hr = -1;
	hr = init_icservice();

	if (!SUCCEEDED(hr))
	{
		return;
	}

	hr = share_work(sharing_manager, virtual_adapter);

	release_icservice();
}