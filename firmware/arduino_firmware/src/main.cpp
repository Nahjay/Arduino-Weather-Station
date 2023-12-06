#include <Arduino.h>
#include <Adafruit_Sensor.h>
#include <DHT.h>
#include <Adafruit_BMP280.h>
#include <RTClib.h>
#include <Wire.h>
#include <BH1750.h>

// Define the variables for the sensors
int DHTPIN = 2;
float temperature = 0;
float humidity = 0;
int setTime = 5000;

// Define macros for the sensors
#define TYPE DHT11

// Create the objects for the sensors
Adafruit_BMP280 bmp; // I2C
RTC_DS3231 rtc;
BH1750 lightMeter(0x23);
DHT dht(DHTPIN, TYPE);


void setup() {
  // put your setup code here, to run once:

  // Start the serial communication
  Serial.begin(9600);

  // Start the sensors
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

  // Set the time
  rtc.adjust(DateTime(F(__DATE__), F(__TIME__)));
  delay(setTime);
}

void loop() {
    // put your main code here, to run repeatedly:

    delay(2000);  // Delay for 2 seconds between readings

    // Read the light sensor
    uint16_t lux = lightMeter.readLightLevel();
    Serial.print("Light: ");
    Serial.print(lux);
    Serial.println(" lx");

    // Read the temperature and humidity sensor
    temperature = dht.readTemperature();
    humidity = dht.readHumidity();

    // Check if any reads failed and exit early (to try again).
    if (isnan(temperature) || isnan(humidity)) {
      Serial.println("Failed to read from DHT sensor!");
      return;
    }

    Serial.print("Temperature: ");
    Serial.print(temperature);
    Serial.println(" °C");
    Serial.print("Humidity: ");
    Serial.print(humidity);
    Serial.println(" %");

    // Read the pressure and altitude sensor
    Serial.print("Pressure = ");
    Serial.print(bmp.readPressure());
    Serial.println(" Pa");
    Serial.print("Altitude: ");
    Serial.print(bmp.readAltitude(1013.25)); // this should be adjusted to your local forcase
    Serial.println(" m");

    // Read the RTC
    DateTime now = rtc.now();
    Serial.print("Time: ");
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

