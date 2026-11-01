import mariadb
import sys

import board
import adafruit_bmp280
i2c = board.I2C()
sensor = adafruit_bmp280.Adafruit_BMP280_I2C(i2c)

print('Temperature: {} degrees C'.format(sensor.temperature)) 
print('Pressure: {}hPa'.format(sensor.pressure))

try:
    conn = mariadb.connect(
        user="db_user",
        password="db_user_passwd",
        host="192.0.2.1",
        port=3306,
        database="employees"

    )
except mariadb.Error as e:
    print(f"Error connecting to MariaDB Platform: {e}")
    sys.exit(1)

cur = conn.cursor()

try:
    cur.execute(
        "INSERT INTO python_data "
        "(emp_no, birth_date, first_name, last_name, gender, hire_date) "
        "VALUES (?, ?, ?, ?, ?, ?)",
        (new_emp_no, "1990-01-15", "Maria", "DB", "F", "2024-01-01"))
except mariadb.Error as e:
    print(f"Error: {e}")

conn.commit()

conn.close()

