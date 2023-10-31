""" Create the API that will be used to interact with the weather station data from the arduino and my website. """

# Import the necessary modules
from fastapi import FastAPI
import uvicorn
import serial


def main():
    """Main function."""

    # Instantiate the FastAPI object
    app = FastAPI()

    # Instantiate the serial object
    ser = serial.Serial("/dev/ttyACM0", 9600)

    # Create the route for the main page
    @app.get("/")
    async def get_root():
        """Return the main page."""

        return {"message": "Hello World"}

    @app.get("/get_temperature")
    async def get_temperature():
        """Return the temperature."""

        return {"temperature": ser.readline()}

    @app.get("/get_humidity")
    async def get_humidity():
        """Return the humidity."""

        return {"humidity": ser.readline()}

    @app.get("/get_pressure")
    async def get_pressure():
        """Return the pressure."""

        return {"pressure": ser.readline()}

    @app.get("/get_altitude")
    async def get_altitude():
        """Return the altitude."""

        return {"altitude": ser.readline()}

    @app.get("/get_light")
    async def get_light():
        """Return the light."""

        return {"light": ser.readline()}

    # Run the server on port 8000 and localhost
    uvicorn.run(app, host="127.0.0.1", port=8000)


if __name__ == "__main__":
    main()
