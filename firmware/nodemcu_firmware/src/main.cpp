#include <Arduino.h>
#include <SoftwareSerial.h>
#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>
#include <vector>

using namespace std;
using std::vector;


// SoftwareSerial mySerial(D6, D5); // RX, TX
const char *ssid = "Family bee";
const char *password = "Kablitv22";
const char *host = "http://localhost:8084/api/temperature";
HTTPClient http;
WiFiClient client;

char buffer[20];
// int index = 0;

void setup()
{
  // put your setup code here, to run once:
  // Begin the Serial at 9600 Baud
  Serial.begin(9600);
  
  while (!Serial) {
    ; // wait for serial port to connect. Needed for native USB port only
  }

  // WiFi.begin(ssid, password);

  // while (WiFi.status() != WL_CONNECTED) {
  //   delay(500);
  //   Serial.println("Connecting to WiFi..");
  // }

  Serial.println("Connected to the WiFi network");
}

void loop()
{
  // Begin Loop

  // Check if there is any data available in the Serial and store it in the buffer
  if (Serial.available() > 0) {
    String data = Serial.readString();
  

    // Check if the data is not empty and if
    if (data != "") {
      // Print the data
      Serial.println(data);

      // Send the data to the server
      http.begin("http://localhost:3000/api/temperature");
      http.addHeader("Content-Type", "application/json");
      int httpCode = http.POST(data);

    }

    




  }


  // Check WiFi connection status
  if (WiFi.status() == WL_CONNECTED) {
    Serial.println("Connected to the WiFi network");
  }

  delay(100);
}
