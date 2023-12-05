#include <Arduino.h>
#include <SoftwareSerial.h>
#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>
#include <vector>

using namespace std;


const char *ssid = "Family bee";
const char *password = "Kablitv22";
const char *host = "http://localhost:8084/weather";
HTTPClient http;
WiFiClient client;


void setup()
{
  // put your setup code here, to run once:
  // Begin the Serial at 9600 Baud
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);

  Serial.begin(9600);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.println("Connecting to WiFi..");
  }

  Serial.println("Connected to ");
  Serial.println(WiFi.SSID());
  Serial.print("IP address:\t");
  Serial.println(WiFi.localIP());
  }
  
  

  // int numSsid = WiFi.scanNetworks();

  // if (numSsid == -1) {
  //   Serial.println("Couldn't get a wifi connection");
  // } 
  // else {
  //   Serial.print("Number of available networks:");
  //   Serial.println(numSsid);      
  // }


  // Initialize the WiFi connection

  // while (WiFi.status() != WL_CONNECTED) {
  //   delay(10000);
  //   WiFi.begin(ssid, password);
  //   Serial.println("Connecting to WiFi..");
  // }

  


void loop()
{
  // Begin Loop

    // Check WiFi connection status
  // if (WiFi.status() == WL_CONNECTED) {
  //   Serial.println("Connected to the WiFi network");
  // }


  // // Check if there is any data available in the Serial and store it in the buffer
  if (Serial.available() > 0) {
    String data = Serial.readString();
  

    // Check if the data is not empty and if
    if (data != "") {
      // Print the data
      Serial.println(data);

      // Send the data to the server
      http.begin(client, host);
      http.addHeader("Content-Type", "application/json");
      int httpCode = http.POST(data);
      String payload = http.getString();
      Serial.println(httpCode);
      Serial.println(payload);
      http.end();
    }

    




  // }
  //  int numSsid = WiFi.scanNetworks();

  // if (numSsid == -1) {
  //   Serial.println("Couldn't get a wifi connection");
  // } 
  // else {
  //   Serial.print("Number of available networks:");
  //   Serial.println(numSsid);      
  // }

  // for (int i = 0; i < numSsid; i++) {
  //   Serial.print("Network name: ");
  //   Serial.println(WiFi.SSID(i));
  //   Serial.print("Signal strength: ");
  //   Serial.println(WiFi.RSSI(i));
  //   Serial.print("MAC address: ");
  //   Serial.println(WiFi.BSSIDstr(i));
  //   Serial.println("-----------------------");
  // }


  delay(100);
}
}
