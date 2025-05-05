package com.filesync.app

import MainScreen
import android.Manifest
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.runtime.mutableStateOf
import androidx.core.content.ContextCompat
import com.filesync.app.hooks.APManager
import com.filesync.app.ui.theme.FileSyncAndroidTheme
import com.journeyapps.barcodescanner.ScanContract
import com.journeyapps.barcodescanner.ScanOptions

class MainActivity : ComponentActivity() {

    private lateinit var apManager: APManager
    private val qrScanResult = mutableStateOf("")
    private val wifiSsid = mutableStateOf("")
    private val wifiPassword = mutableStateOf("")

    // QR Code Scanner Launcher
    private val qrCodeLauncher = registerForActivityResult(ScanContract()) { result ->
        if (result.contents == null) {
            Toast.makeText(this, "Scan cancelled", Toast.LENGTH_SHORT).show()
        } else {
            qrScanResult.value = result.contents
        }
    }

    // Location Permission Request
    private val locationPermissionRequest = registerForActivityResult(
        ActivityResultContracts.RequestPermission()
    ) { isGranted ->
        if (isGranted) {
            checkWriteSettingsPermission()
        } else {
            Toast.makeText(this, "Location permission is required", Toast.LENGTH_SHORT).show()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        apManager = APManager.getApManager(this)

        if (ContextCompat.checkSelfPermission(this, Manifest.permission.ACCESS_FINE_LOCATION)
            != PackageManager.PERMISSION_GRANTED
        ) {
            locationPermissionRequest.launch(Manifest.permission.ACCESS_FINE_LOCATION)
        } else {
            checkWriteSettingsPermission()
        }

        setContent {
            FileSyncAndroidTheme {
                MainScreen(
                    qrResult = qrScanResult.value,
                    onScanClick = { checkCameraPermission(this) },
                    wifiSsid = wifiSsid.value,
                    wifiPassword = wifiPassword.value
                )
            }
        }
    }

    private fun checkCameraPermission(context: Context) {
        when {
            ContextCompat.checkSelfPermission(context, Manifest.permission.CAMERA) ==
                    PackageManager.PERMISSION_GRANTED -> launchQrScanner()

            shouldShowRequestPermissionRationale(Manifest.permission.CAMERA) -> {
                Toast.makeText(
                    this,
                    "Camera permission is needed to scan QR codes",
                    Toast.LENGTH_SHORT
                ).show()
            }

            else -> {
                requestPermissions(arrayOf(Manifest.permission.CAMERA), 100)
            }
        }
    }

    private fun checkWriteSettingsPermission() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && !Settings.System.canWrite(this)) {
            val intent = Intent(Settings.ACTION_MANAGE_WRITE_SETTINGS).apply {
                data = Uri.parse("package:$packageName")
            }
            startActivity(intent)
        } else {
            turnOnHotspot()
        }
    }

    private fun launchQrScanner() {
        val options = ScanOptions().apply {
            setDesiredBarcodeFormats(ScanOptions.QR_CODE)
            setPrompt("Scan QR code")
            setCameraId(0)
            setBeepEnabled(false)
            setOrientationLocked(false)
        }
        qrCodeLauncher.launch(options)
    }

    private fun turnOnHotspot() {
        apManager.turnOnHotspot(
            this,
            object : APManager.OnSuccessListener {
                override fun onSuccess(ssid: String, password: String) {
                    Log.d("Hotspot", "SSID: $ssid, Password: $password")
                    wifiSsid.value = ssid
                    wifiPassword.value = password

                }
            },
            object : APManager.OnFailureListener {
                override fun onFailure(failureCode: Int, e: Exception?) {
                    val message = when (failureCode) {
                        APManager.ERROR_GPS_PROVIDER_DISABLED -> "Enable GPS to start hotspot"
                        APManager.ERROR_LOCATION_PERMISSION_DENIED -> "Grant location permission"
                        APManager.ERROR_DISABLE_HOTSPOT -> "Disable existing hotspot first"
                        APManager.ERROR_DISABLE_WIFI -> "Turn off Wi-Fi to create hotspot"
                        APManager.ERROR_WRITE_SETTINGS_PERMISSION_REQUIRED -> "Allow modify system settings"
                        else -> "Unknown error: ${e?.message}"
                    }

                    Toast.makeText(this@MainActivity, "Hotspot failed: $message", Toast.LENGTH_LONG)
                        .show()
                    e?.printStackTrace()
                }
            }
        )
    }
}
