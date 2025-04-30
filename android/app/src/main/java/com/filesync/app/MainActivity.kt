package com.filesync.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
//import androidx.compose.foundation.layout.FlowRowScopeInstance.weight
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.BottomAppBar
import androidx.compose.material3.BottomAppBarDefaults
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.filesync.app.ui.theme.FileSyncAndroidTheme
import androidx.compose.material3.Icon
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material3.Card
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.FloatingActionButtonDefaults
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.filesync.app.screens.WelcomeScreen
import com.filesync.app.ui.theme.Accent


class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            FileSyncAndroidTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                ) {
                    Box {
                        Column() {
                            WelcomeScreen()
                        }
                    }
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Composable
fun Header() {

}

@Composable
fun MemoryCard(modifier: Modifier = Modifier) {
    Card(Modifier.padding(10.dp)) {
        Column(
            Modifier
                .padding(vertical = 15.dp)
                .height(100.dp)
                .fillMaxWidth()
        ) {
            Text("Available memory", fontWeight = FontWeight.Bold, fontSize = 24.sp)
            Text("25 Gb of 324 Gb")
        }
    }
}

@Composable
fun Tabs(selectedTabIndex: Int = 0, modifier: Modifier = Modifier) {
    val tabs = listOf<String>(
        "History",
        "Application",
        "Video",
        "Audio",
        "Pictures",
        "Document",
        "File manager",
        "Large filesÏ"
    )
    Row(modifier) {
        for (tab in tabs) {
            Text(tab, Modifier.padding(10.dp))
        }
    }
}