# Preface
The [Mosquitto MQTT broker](https://mosquitto.org) provides a number of statistics on the special `$SYS/#` topic (see [mosquitto(8)](https://mosquitto.org/man/mosquitto-8.html)).

# Repositories
* Primary development repository: https://git.ypbind.de/cgit/prometheus-mosquitto-exporter/
* Backup repository: https://github.com/Bobobo-bo-Bo-bobo/prometheus-mosquitto-exporter

# Build requirements
As a Rust programm, a working stable Rust toolchain with Rust *version 1.50 or newer* is required.

Additionally, `cmake`, `pkg-config` and the `libssl` development package are required by the `paho-mqtt` crate.

# Configuration
## Command line parameters
The exporter supports the following command line parameters:

| *Option* | *Description* | *Note* |
|:------------|:--------|:-------|
| `-V` / `--version` | Show version information | |
| `-c <config>` / `--config=<config>`| Path to the configuration file | **mandatory** |
| `-h` / `--help` | Show help text | |
| `-q` / `--quiet` | Log only warning and error messages | |

## Configuration file
The configuration file is a YAML file defining the exporter and MQTT connection parameters:

```yaml
---
service:
  # Address to listen on, default: localhost:9883
  listen: 'localhost:9883'
  # URL path for metrics exposure, default: /metrics
  metrics_path: '/metrics'
mqtt:
  # MQTT authentication
  auth:
    # MQTT user with read permissions on the $SYS/# topic
    user: 'mqtt_user'
    # Password of the MQTT user
    password: 'mqtt_password'
  # MQTT broker address, scheme://address valid schemes are tcp and ssl
  broker: 'tls://mqtt.name.or.addr.ess'
  # CA file to use for server certificate validation, default: /etc/ssl/certs/ca-certificates.crt
  ca_file: '/etc/ssl/certs/ca-certificates.crt'
  # Disable verification of servers SSL certificate
  insecure_ssl: false
  # maximal time in seconds for reconnect in seconds
  retry_interval: 60
  # timeout for connection establishment in seconds
  timeout: 5
  # QoS, default: 0
  qos: 0
  # Client ID for MQTT connection, default: mosquitto-exporter
  client_id: 'mqtt_client_id'
```

# Licenses
## prometheus-mosquitto-exporter
Copyright (C) 2021 by Andreas Maus

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Eclipse Paho MQTT Rust Client Library

Eclipse Distribution License - v 1.0

Copyright (c) 2007, Eclipse Foundation, Inc. and its licensors.

All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
    Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
    Neither the name of the Eclipse Foundation, Inc. nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission. 

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


