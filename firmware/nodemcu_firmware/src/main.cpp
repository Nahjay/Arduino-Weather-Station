#include <Arduino.h>
#include <SoftwareSerial.h>
#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>

// SoftwareSerial mySerial(D6, D5); // RX, TX  

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  // mySerial.begin(9600);
  while (!Serial) {
    ; // wait for serial port to connect. Needed for native USB port only
  }
  
}

void loop() {
  // put your main code here, to run repeatedly:
  if (Serial.available() > 0) {
    Serial.write(Serial.read());
  }
  else {
    Serial.println("No data");
  }
  // if (mySerial.available() > 0) {
  //   Serial.write(mySerial.read());
  // }
  // else {
  //   Serial.println("No data");
  // }

  delay(100);
}
