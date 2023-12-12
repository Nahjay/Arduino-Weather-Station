#include <SoftwareSerial.h>
#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>
#include <ArduinoJson.h>


const char *ssid = "Family bee_EXT";
const char *password = "Kablitv22";
const char *host = "http://192.168.0.105:8084/post_weather";
WiFiClient client;


void setup() {
  // Begin the Serial at 9600 Baud
  WiFi.mode(WIFI_STA);

  delay(100);
  WiFi.begin(ssid, password);

  Serial.begin(9600);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.println("Connecting to WiFi..");
  }

  Serial.println("Connected to ");
  Serial.println(WiFi.SSID());

  // Delay for 1 sec
  delay(1000);

  IPAddress staticIP(192, 168, 0, 184); //ESP static ip
  IPAddress gateway(192, 168, 0, 1);   //IP Address of your WiFi Router (Gateway)
  IPAddress subnet(255, 255, 255, 0);  //Subnet mask

  // Set your Static IP address
  if (!WiFi.config(staticIP, gateway, subnet)) {
    Serial.println("STA Failed to configure");
  }

  // Print ESP8266 Local IP Address
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
        DynamicJsonDocument doc(1024);

        // Add the variable to the JSON document
        doc["data"] = data;

        // Convert the JSON document to a string
        String output;
        serializeJson(doc, output);
        Serial.println(output);

        // Print the JSON document
        Serial.println("JSON document:\n");
        Serial.println(output);

        // Send the request
        HTTPClient http;

        // Begin the request
        http.begin(client, host);

        http.addHeader("Content-Type", "application/json");

        // Send the request
        int httpResponseCode = http.POST(output);
        Serial.print("HTTP Response code: ");
        Serial.println(httpResponseCode);
        Serial.print("HTTP Response body: ");
        Serial.println(http.getString());

        // Free resources       
        http.end();
      }
    }
  }
  else {
    Serial.println("Error in WiFi connection");
  }    
  delay(3000);
}

