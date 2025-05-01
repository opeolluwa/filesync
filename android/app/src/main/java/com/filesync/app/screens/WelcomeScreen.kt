package com.filesync.app.screens

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Arrangement
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
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.filesync.app.R
import com.filesync.app.components.PulsingCirclesAnimation
import com.filesync.app.ui.theme.Accent
import com.filesync.app.ui.theme.Accent200
import com.filesync.app.ui.theme.Accent50
import com.filesync.app.ui.theme.Accent600


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
            modifier = Modifier.fillMaxWidth()
        ) {
            Text(
                "Waiting for connection",
                fontSize = 24.sp,
                fontWeight = FontWeight(500),
                lineHeight = 24.sp,
                textAlign = TextAlign.Center,
                modifier = Modifier.padding(top = 12.dp)
            )
            Text("Scan QR code on peer device to continue")

        }
        Button(
            onClick = { scanQRCode() },
            shape = RoundedCornerShape(5.dp),
            colors = ButtonDefaults.buttonColors(
                contentColor = Color.White,
                containerColor = Accent

            ),
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .fillMaxWidth(0.8f)
                .padding(bottom = 48.dp)
                .height(48.dp)
        ) {
            Text("Scan QR code", fontSize = 16.sp)
            Image(
                painter = painterResource(id = R.drawable.qr_code),
                contentDescription = "Custom Vector Icon",
                modifier = Modifier
                    .size(24.dp)
                    .padding(horizontal = 2.dp)
            )
        }
    }


}

fun scanQRCode(): UInt {
    val value: UInt = 5u;
    return value
}