#include <Arduino.h>
#include <SoftwareSerial.h>
#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>
#include <vector>
#include <ArduinoJson.h>

using namespace std;


const char *ssid = "Family bee";
const char *password = "Kablitv22";
const char *host = "http://localhost:8084/weather";
HTTPClient http;
WiFiClient client;


void setup() {
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


void loop() {
  // Begin Loop


  // Check if the WiFi is connected
  if (WiFi.status() == WL_CONNECTED) {
    Serial.println("Connected to the WiFi network");
  

    // // Check if there is any data available in the Serial and store it in the buffer
    if (Serial.available() > 0) {
      String data = Serial.readString();

      // Check if the data is not empty and if not, send it to the server
      if (data != "") {
        // Print the data
        Serial.println(data);

        // Create a JSON document
        DynamicJsonDocument doc(10000);

        // Parse the JSON document
        deserializeJson(doc, data);

        // Convert the JSON document to a string
        String output;
        serializeJson(doc, output);

        // Print the JSON document
        Serial.println("JSON document:\n");
        Serial.println(output);

        // Send the request to the server
        http.begin(client, host);
        http.addHeader("Content-Type", "application/json");

        // Send the request and get the response
        int httpResponseCode = http.POST(output);
        Serial.print("HTTP Response code: ");
        Serial.println(httpResponseCode);
      }

    }
  }
  else {
    Serial.println("Error in WiFi connection");
  }
    

  delay(1000);
}

