from scapy.all import *

HERMOD_PORT = 4444
SFTP_PORT = 22


def hermod_len():
    scapy_cap = rdpcap('output/hermod.pcap')
    bytes_sent = 0
    bytes_received = 0
    packets_sent = 0
    packets_received = 0
    for packet in scapy_cap:
        if TCP not in packet.layers():
            continue
        if packet[TCP].dport == HERMOD_PORT:
            bytes_sent += packet.payload.len
            packets_sent += 1
        if packet[TCP].sport == HERMOD_PORT:
            bytes_received += packet.payload.len
            packets_received += 1
    return [packets_sent, packets_received, bytes_sent, bytes_received]


def sftp_len(file_name):
    scapy_cap = rdpcap('output/{}.pcap'.format(file_name))
    bytes_sent = 0
    bytes_received = 0
    packets_sent = 0
    packets_received = 0
    for packet in scapy_cap:
        if TCP not in packet.layers():
            continue
        if packet[TCP].dport == SFTP_PORT:
            bytes_sent += packet.payload.len
            packets_sent += 1
        if packet[TCP].sport == SFTP_PORT:
            bytes_received += packet.payload.len
            packets_received += 1

    return [packets_sent, packets_received, bytes_sent, bytes_received]


hermod_res = hermod_len()
scp_res = sftp_len('scp')
sftp_res = sftp_len('sftp')

print("Hermod:")
print("\tpackets sent: {}".format(hermod_res[0]))
print("\tpackets received: {}".format(hermod_res[1]))
print("\tbytes packets sent: {}".format(hermod_res[2]))
print("\tbytes packets received: {}".format(hermod_res[3]))
print("\tTotal packets sent: {}".format(hermod_res[0] + hermod_res[1]))
print("\tTotal bytes sent: {}".format(hermod_res[2] + hermod_res[3]))

print("SCP:")
print("\tpackets sent: {}".format(scp_res[0]))
print("\tpackets received: {}".format(scp_res[1]))
print("\tbytes packets sent: {}".format(scp_res[2]))
print("\tbytes packets received: {}".format(scp_res[3]))
print("\tTotal packets sent: {}".format(scp_res[0] + scp_res[1]))
print("\tTotal bytes sent: {}".format(scp_res[2] + scp_res[3]))

print("SFTP:")
print("\tpackets sent: {}".format(sftp_res[0]))
print("\tpackets received: {}".format(sftp_res[1]))
print("\tbytes packets sent: {}".format(sftp_res[2]))
print("\tbytes packets received: {}".format(sftp_res[3]))
print("\tTotal packets sent: {}".format(sftp_res[0] + sftp_res[1]))
print("\tTotal bytes sent: {}".format(sftp_res[2] + sftp_res[3]))
