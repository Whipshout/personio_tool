# Personio tool

# Outdated! Personnio changed the request and added more security :')

## How to use

- Download personio_tool_mac.zip/personio_tool_windows.zip, extract the folder, update config.json (example below), run app in the terminal.<br>

- if ``"untilToday": false``, app will use startDay and endDay to fill the calendar.<br>

- if ``"untilToday": true``, app will fill days backwards from today until the last worked filled day.
<br>
<br>

{<br>
&emsp;"credentials": {<br>
&emsp;&emsp;"email": "your_email@personio",<br>
&emsp;&emsp;"password": "your_password"<br>
&emsp;},<br>
&emsp;"dates": {<br>
&emsp;&emsp;"startDay": "2022-01-24",<br>
&emsp;&emsp;"endDay": "2022-01-26",<br>
&emsp;&emsp;"untilToday": false<br>
&emsp;},<br>
&emsp;"times": {<br>
&emsp;&emsp;"breakStartHour": "14:00:00.0",<br>
&emsp;&emsp;"breakDurationMinutes": 30,<br>
&emsp;&emsp;"breakRandomMinutesDelta": 30,<br>
&emsp;&emsp;"workStartHour": "08:00:00.0",<br>
&emsp;&emsp;"workDurationHours": 8,<br>
&emsp;&emsp;"workRandomMinutesDelta": 30<br>
&emsp;},<br>
&emsp;"params": {<br>
&emsp;&emsp;"delayBetweenRequestsMs": 1000,<br>
&emsp;&emsp;"timeoutRequestSeconds": 20<br>
&emsp;}<br>
}
<br>
<br>
- Open the terminal, execute the file
    - Windows: ``.\personio_tool``
    - Mac: ``./personio_tool``
