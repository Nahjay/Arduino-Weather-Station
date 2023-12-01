#include <Arduino.h>
#include <Adafruit_Sensor.h>
#include <DHT.h>

#define TYPE DHT11
int DHTPIN = 2;
DHT dht(DHTPIN, TYPE);
float temperature = 0;
float humidity = 0;
int setTime = 5000;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  dht.begin();
  delay(setTime);
}

void loop() {
    // put your main code here, to run repeatedly:

    delay(2000);  // Delay for 2 seconds between readings

    temperature = dht.readTemperature();
    humidity = dht.readHumidity();

    // Check if any reads failed and exit early (to try again).
    if (isnan(temperature) || isnan(humidity)) {
      Serial.println("Failed to read from DHT sensor!");
      return;
    }

    Serial.print("Temperature: ");
    Serial.print(temperature);
    Serial.print(" °C, Humidity: ");
    Serial.print(humidity);
    Serial.println(" %");
}

