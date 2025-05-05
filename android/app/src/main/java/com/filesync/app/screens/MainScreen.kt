import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.filesync.app.R
import com.filesync.app.components.PulsingCirclesAnimation
import com.filesync.app.components.WifiConfigModal
import com.filesync.app.ui.theme.Accent


@Composable
fun MainScreen(qrResult: String, onScanClick: () -> Unit, wifiSsid: String, wifiPassword: String) {
    val showDialog = remember { mutableStateOf(false) }



    if (showDialog.value) {
        WifiConfigModal(
            wifiSsid = "MyHotspot",
            wifiPassword = "superSecret123",
            onConfirm = { showDialog.value = false },
            onDismiss = { showDialog.value = false }
        )
    }


    Surface(modifier = Modifier.fillMaxSize()) {
        Box {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(16.dp)
            ) {
                Box(modifier = Modifier.weight(1f)) {
                    Column(
                        modifier = Modifier
                            .align(Alignment.Center)
                            .padding(bottom = 50.dp),
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        PulsingCirclesAnimation()
                    }
                }

                Column(
                    horizontalAlignment = Alignment.CenterHorizontally,
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(bottom = 48.dp)
                ) {
                    Text(
                        "Waiting for connection...",
                        fontSize = 16.sp,
                        fontWeight = FontWeight.Medium,
                        textAlign = TextAlign.Center
                    )


                    Button(
                        onClick = onScanClick,
                        shape = RoundedCornerShape(5.dp),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = Accent,
                            contentColor = Color.White
                        ),
                        border = BorderStroke(1.dp, Accent),
                        modifier = Modifier
                            .fillMaxWidth(0.7f)
                            .padding(top = 12.dp)
                            .height(48.dp)
                            .shadow(8.dp, RoundedCornerShape(50), clip = false)
                    ) {
                        Text("Scan QR code", fontSize = 14.sp)
                        Image(
                            painter = painterResource(id = R.drawable.qr_code),
                            contentDescription = "QR Code Icon",
                            modifier = Modifier.size(24.dp)
                        )
                    }
                }
            }
        }
    }
}
