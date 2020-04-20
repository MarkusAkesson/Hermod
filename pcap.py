from scapy.all import *

HERMOD_PORT = 4444
SFTP_PORT = 22


def hermod_len():
    scapy_cap = rdpcap('output/hermod.pcap')
    transfer_len = 0
    for packet in scapy_cap:
        if TCP not in packet.layers():
            continue
        if packet[TCP].dport != HERMOD_PORT:
            continue
        transfer_len += packet.payload.len
    return transfer_len


def sftp_len(file_name):
    scapy_cap = rdpcap('output/{}.pcap'.format(file_name))
    transfer_len = 0
    for packet in scapy_cap:
        if TCP not in packet.layers():
            continue
        if packet[TCP].dport != SFTP_PORT:
            continue
        transfer_len += packet.payload.len
    return transfer_len


print("Hermod session length: {}".format(hermod_len()))
print("SCP session length: {}".format(sftp_len('scp')))
print("SFTP session length: {}".format(sftp_len('sftp')))
