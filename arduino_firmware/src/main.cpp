#include <Arduino.h>
#include <Adafruit_Sensor.h>
#include <DHT.h>
#include <Adafruit_BMP280.h>
#include <RTClib.h>
#include <Wire.h>
#include <BH1750.h>

Adafruit_BMP280 bmp; // I2C
RTC_DS3231 rtc;

#define TYPE DHT11
int DHTPIN = 2;
DHT dht(DHTPIN, TYPE);
float temperature = 0;
float humidity = 0;
int setTime = 5000;
BH1750 lightMeter(0x23);


void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  dht.begin();
  Wire.begin();
  lightMeter.begin(BH1750::CONTINUOUS_HIGH_RES_MODE);
  if (!bmp.begin(0x76)) {
    Serial.println(F("Could not find a valid BMP280 sensor, check wiring!"));
    while (1);
  }
  if (! rtc.begin()) {
    Serial.println("Couldn't find RTC");
    while (1);
  }
  rtc.adjust(DateTime(F(__DATE__), F(__TIME__)));
  delay(setTime);
}

void loop() {
    // put your main code here, to run repeatedly:



    delay(2000);  // Delay for 2 seconds between readings


    uint16_t lux = lightMeter.readLightLevel();
    Serial.print("Light: ");
    Serial.print(lux);
    Serial.println(" lx");

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

    Serial.print("Temperature = ");
    Serial.print(bmp.readTemperature());
    Serial.println(" *C");
    Serial.print("Pressure = ");
    Serial.print(bmp.readPressure());
    Serial.println(" Pa");
    Serial.print("Approx altitude = ");
    Serial.print(bmp.readAltitude(1013.25)); // this should be adjusted to your local forcase
    Serial.println(" m");
    Serial.println();

    DateTime now = rtc.now();

    Serial.print(now.year(), DEC);
    Serial.print('/');
    Serial.print(now.month(), DEC);
    Serial.print('/');
    Serial.print(now.day(), DEC);
    Serial.print(" ");
    Serial.print(now.hour(), DEC);
    Serial.print(':');
    Serial.print(now.minute(), DEC);
    Serial.print(':');
    Serial.print(now.second(), DEC);
    Serial.println();
    delay(setTime); // Delay for 5 seconds.

}

