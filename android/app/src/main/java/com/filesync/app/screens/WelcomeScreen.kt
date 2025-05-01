package com.filesync.app.screens

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
import androidx.compose.material3.ButtonElevation
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.filesync.app.R
import com.filesync.app.components.PulsingCirclesAnimation
import com.filesync.app.ui.theme.Accent


@Composable()

fun WelcomeScreen(modifier: Modifier = Modifier) {
    Box(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {


        Column(
            modifier = Modifier
                .align(Alignment.Center)
                .padding(bottom = 50.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            PulsingCirclesAnimation()
        }


        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 48.dp)
                .align(Alignment.BottomCenter)
        ) {
            Text(
                "Waiting for connection...",
                fontSize = 16.sp,
                fontWeight = FontWeight(500),
                lineHeight = 24.sp,
                textAlign = TextAlign.Center,
                modifier = Modifier.padding(top = 12.dp)
            )



            Button(
                onClick = { scanQRCode() },
                shape = RoundedCornerShape(5.dp),
                colors = ButtonDefaults.buttonColors(
                    containerColor = Accent,
                    contentColor = Color.White
                ),
                border = BorderStroke(1.02.dp, Accent),
                modifier = Modifier
                    .fillMaxWidth(0.7f)
                    .padding(top = 12.dp)
                    .height(48.dp).shadow(8.dp, RoundedCornerShape(50), clip = false)
            ) {
                Text("Scan QR code", fontSize = 14.sp)
                Image(
                    painter = painterResource(id = R.drawable.qr_code),
                    contentDescription = "Custom Vector Icon",
                    modifier = Modifier
                        .size(24.dp)
                        .padding(horizontal = 2.dp),
//                    colorFilter = ColorFilter.tint(Accent)
                )
            }
        }
    }


}

fun scanQRCode(): UInt {
    val value: UInt = 5u
    return value
}