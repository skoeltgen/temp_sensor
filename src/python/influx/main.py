import board
import adafruit_bmp280
import influxdp_client_3
from influxdb_client_3 import InfluxDBClient3, Point, WriteOptions

# Define BMP280 Input Device
i2c = board.I2C()
sensor = adafruit_bmp280.Adafruit_BMP280_I2C(i2c)

print('Temperature: {} degrees C'.format(sensor.temperature)) 
print('Pressure: {}hPa'.format(sensor.pressure))

# Define InfluxDB Data
bucket = "<my-bucket>"
org = "<my-org>"
token = "<my-token>"
# Store the URL of your InfluxDB instance
url="http://localhost:8086"

# Init Client
client = InfluxDBClient3(host=f"localhost:8181",database=f"PYTHON_DATA",token=f"DATABASE_TOKEN")
# Create Data to write to Database
time = datetime.time_ns()
data_point = f"python,location=Test temperature={sensor.temperature},pressure={sensor.pressure} time"
