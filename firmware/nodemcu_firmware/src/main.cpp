#include <Arduino.h>
#include <SoftwareSerial.h>
#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>
#include <vector>

using namespace std;
using std::vector;


const char *ssid = "Family bee";
const char *password = "Kablitv22";
const char *host = "http://localhost:8084/weather";
HTTPClient http;
WiFiClient client;


void setup()
{
  // put your setup code here, to run once:
  // Begin the Serial at 9600 Baud
  Serial.begin(9600);
  
  while (!Serial) {
    ; // wait for serial port to connect. Needed for native USB port only
  }

  // Initialize the WiFi connection
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    WiFi.begin(ssid, password);
    Serial.println("Connecting to WiFi..");
  }

  Serial.println("Connected to the WiFi network");
}

void loop()
{
  // Begin Loop

    // Check WiFi connection status
  if (WiFi.status() == WL_CONNECTED) {
    Serial.println("Connected to the WiFi network");
  }


  // Check if there is any data available in the Serial and store it in the buffer
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

    




  }

  delay(100);
}
