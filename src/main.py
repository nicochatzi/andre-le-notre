#!/usr/bin/python3
import sys
import time
import json
from datetime import datetime
import requests

IRRIGATION_HOUR = 7
IRRIGATION_FREQUENCY = {
    'summer': 2,
    'winter': 3,
}
SUMMER = {
    'start': 4,
    'end': 8,
}
FLOW_METER_PIN = 0
STATIONS = [
    {
        'pin_num': 0,
        'irrigation_length': 15,
        'pressure_threshold': 100
    },
    {
        'pin_num': 0,
        'irrigation_length': 15,
        'pressure_threshold': 100
    },
    {
        'pin_num': 0,
        'irrigation_length': 15,
        'pressure_threshold': 100
    },
    {
        'pin_num': 0,
        'irrigation_length': 15,
        'pressure_threshold': 100
    },
]
PRECIPITATION_THRESHOLD = {
    'probability': 80,
    'amount': 50,
}


class EmailClient:
    def __init__(self):
        pass

    def notify_broken_stations(self, stations):
        pass

    def notify_http_error(self, error):
        pass

    def _send(self, email):
        pass


class IrrigationStation:
    def __init__(self, pin_num, pressure_threshold, irrigation_time):
        self.pin_num = pin_num
        self.pressure_threshold = pressure_threshold
        self.irrigation_time = irrigation_time
        self.is_broken = False

    def test(self):
        return True

    def run(self):
        pass


class IrrigationController:
    def __init__(self):
        self.stations = []
        for i in range(len(STATIONS)):
            station = STATIONS[i]
            self.stations.append(IrrigationStation(
                station['pin_num'],
                station['pressure_threshold'],
                station['irrigation_time']))

    def _test_station(self, station):
        return True

    def test(self):
        newly_broken_stations = []
        for station in self.stations:
            if not station.is_broken:
                self._test_station(station)
                if station.is_broken:
                    newly_broken_stations.append(station)
        return newly_broken_stations

    def run(self):
        return True


def parse_weather_code(code):
    return {
        "0": "Unknown",
        "1000": "Clear",
        "1001": "Cloudy",
        "1100": "Mostly Clear",
        "1101": "Partly Cloudy",
        "1102": "Mostly Cloudy",
        "2000": "Fog",
        "2100": "Light Fog",
        "3000": "Light Wind",
        "3001": "Wind",
        "3002": "Strong Wind",
        "4000": "Drizzle",
        "4001": "Rain",
        "4200": "Light Rain",
        "4201": "Heavy Rain",
        "5000": "Snow",
        "5001": "Flurries",
        "5100": "Light Snow",
        "5101": "Heavy Snow",
        "6000": "Freezing Drizzle",
        "6001": "Freezing Rain",
        "6200": "Light Freezing Rain",
        "6201": "Heavy Freezing Rain",
        "7000": "Ice Pellets",
        "7101": "Heavy Ice Pellets",
        "7102": "Light Ice Pellets",
        "8000": "Thunderstorm"
    }.get(code, 'Invalid Weather code')


def num_forecast_days():
    month = datetime.now().month
    if SUMMER['start'] <= month <= SUMMER['end']:
        return IRRIGATION_FREQUENCY['summer']
    return IRRIGATION_FREQUENCY['winter']


class HttpError(Exception):
    def __init__(self, code, message):
        self.code = code
        self.message = message
        super().__init__(self.code)


class WeatherReport:
    def __init__(self, is_real_api=True):
        self.is_real_api = is_real_api
        self.report = json.load(open('../res/data.json'))
        self.precipitation = {
            'probability': 0.0,
            'amount': 0.0,
        }

    def update(self):
        if not self.is_real_api:
            self.repost = self.report['data']['timelines'][0]['intervals']
            self._store_precipitation()
        else:
            response = requests.get(
                'GET',
                'https://api.tomorrow.io/v4/timelines',
                params={
                    'location': '6.77626,43.166',
                    'units': 'metric',
                    'fields': 'temperature',
                    'fields': 'temperatureApparent',
                    'fields': 'weatherCode',
                    'fields': 'precipitationIntensity',
                    'fields': 'precipitationProbability',
                    'timestep': '1d',
                    'apikey': 'dgcK0rhxRVBpmr5xbnB9fwzEYvLEifxO',
                },
                headers={'Accept': 'application/json'}
            )

            if 200 <= response.status_code < 300:
                self.report = response.json(
                )['data']['timelines'][0]['intervals']
                self._store_precipitation()
            else:
                raise HttpError(response.status_code, response.text)

    def _store_precipitation(self):
        num_days = num_forecast_days()

        if len(self.report) < num_days:
            raise Exception

        amount = 0.0
        probability = 0.0
        for day in range(num_days):
            probability += day['probability'] / num_days
            amount += day['amount']

        self.precipitation = {
            'probability': self.report[0]['precipitationProbability'],
            'amount': self.report[0]['precipitationIntensity'],
        }

    def does_forecast_rain(self):
        return self.precipitation['probability'] > PRECIPITATION_THRESHOLD['probability'] \
            and self.precipitation['amount'] > PRECIPITATION_THRESHOLD['amount']

    def print(self):
        print(f"@ {datetime.now()}")
        for reading in self.report:
            print(f"""  {reading['startTime']} : {parse_weather_code(reading['weatherCode'])}
    Temperature   [
        Real      : {reading['temperature']}C
        Apparent  : {reading['temperatureApparent']}C
    ]
    Precipitation [
        Probability : {reading['precipitationProbability']}%
        Intensity   : {reading['precipitationIntensity']}mm/hr
    ]
""")


def run(controller, notifier, previous_report, sleep_time_s=60 * 60, use_test_api=False):
    try:
        print('~~> Testing irrigation system')

        newly_broken_stations = controller.test()
        if len(newly_broken_stations) != 0:
            notifier.notify_broken_stations(newly_broken_stations)

        print('~~> Getting weather report')

        current_report = WeatherReport(is_real_api=use_test_api)
        print(current_report.print())

        if not previous_report.does_forecast_rain():
            if not current_report.does_forecast_rain():
                controller.run()

        print('~~> Storing this weather report')
        current_report = previous_report

    except HttpError as err:
        notifier.notify_http_error(err)

    except Exception as err:  # pylint: disable=broad-except
        print('~~> Uncaught error')

    finally:
        time.sleep(sleep_time_s)


def is_time_to_run(last_run):
    return today.hour == IRRIGATION_HOUR \
        and last_run.day < today.day


def is_test_run():
    if len(sys.argv) > 2:
        return sys.argv[2] == 'test'
    return False


if __name__ == '__main__':
    is_test = is_test_run()
    email_client = EmailClient()
    irrigation_controller = IrrigationController()
    previous_weather_report = WeatherReport(is_test)
    last_run_time = datetime.fromtimestamp(0)

    while True:
        today = datetime.today()

        if is_time_to_run(last_run_time):
            last_run_time = today
            run(irrigation_controller, email_client,
                previous_weather_report, is_test)
