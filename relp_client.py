import argparse
import socket
import time
import numpy as np

RELP_OPEN_COMMAND = "1 open 30 relp_version=0 commands=syslog"
RELP_CLOSE_COMMAND = "{tx_number} close 0"
RELP_SYSLOG_MESSAGE = "{tx_number} syslog 12 abcdefghijkl"

def open_connection(ip, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((ip, port))
    return s

def close_connection(soc):
    soc.close()


def send(s, data):
    print "sending", data
    s.send(data)
    my_bytes = bytearray()
    my_bytes.append("\n")
    s.send(my_bytes)
    data = s.recv(1024)
    print data


def run(ip, port, load, count):
    soc = open_connection(ip, port)

    send(soc, RELP_OPEN_COMMAND)
    for i in range(2, count):
        print "sending record: ", i

        send(soc,RELP_SYSLOG_MESSAGE.format(**{"tx_number": i}))

        sleep_time = 1.0 / ((np.random.poisson(load * 10000) * 1.0) / 10000)
        time.sleep(sleep_time)

    send(soc, RELP_CLOSE_COMMAND.format(**{"tx_number": count +1}))
    close_connection(soc)

parser = argparse.ArgumentParser(description='RELP Test client')

# Connection info
parser.add_argument('--ip', dest='ip', help='Relp server ip', default='localhost')
parser.add_argument('--port', type=int, dest='port', help='Port relp server is listeing on',
                    default=12345)
parser.add_argument('--periodicity', type=int, dest='periodicity', help='Seconds between records',
                    default=0.1)
parser.add_argument('--iterations', type=int, dest='iterations', help='How many records',
                    default=1000)

args = parser.parse_args()

run(args.ip, args.port, args.periodicity, args.iterations)

