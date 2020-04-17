import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import json


def load_means(files):
    means = []
    for file in files:
        with open(file) as f:
            a = []
            raw_json = json.loads(f.read())
            for i in range(0, 4):
                mean = raw_json['results'][i]['mean']
                a.append(round(mean, 3))
        means.append(a)
    return means


labels = ['1G', '500M', '10K', 'Hermod source code']
data_files = ['output/hermod.json', 'output/scp.json', 'output/sftp.json']
means = load_means(data_files)
hermod_means = means[0]
scp_means = means[1]
sftp_means = means[2]

x = np.arange(len(labels))  # the label locations
width = 0.3  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(x - width, hermod_means, width, label='Hermod')
rects2 = ax.bar(x, scp_means, width, label='scp')
rects3 = ax.bar(x + width, sftp_means, width, label='sftp')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Seconds')
ax.set_xlabel('File size')
ax.set_title('Comparison between Hermod, scp and sftp')
ax.set_xticks(x)
ax.set_xticklabels(labels)
ax.legend()


def autolabel(rects):
    """Attach a text label above each bar in *rects*, displaying its height."""
    for rect in rects:
        height = rect.get_height()
        ax.annotate('{}'.format(height),
                    xy=(rect.get_x() + rect.get_width() / 3, height),
                    xytext=(0, 3),  # 3 points vertical offset
                    textcoords="offset points",
                    ha='center', va='bottom')


autolabel(rects1)
autolabel(rects2)
autolabel(rects3)

fig.tight_layout()

plt.show()
