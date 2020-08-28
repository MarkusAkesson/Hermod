import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import json


def load_stats(apps, n_files):
    stats = {}
    for app in apps:
        path = "output/{}.json".format(app)
        with open(path) as f:
            a = {'mean': [], 'max': [], 'min': [], 'stddev': []}
            raw_json = json.loads(f.read())
            for i in range(0, n_files):
                a['mean'].append(round(raw_json['results'][i]['mean'], 2))
                a['max'].append(round(raw_json['results'][i]['max'], 2))
                a['min'].append(round(raw_json['results'][i]['min'], 2))
                a['stddev'].append(round(raw_json['results'][i]['stddev'], 2))
            stats[app] = a
    return stats


def as_table(stats, apps, labels):
    print("\\begin{tabular}{ | l | c | c | c | c | c | } \\hline")
    print("{} & {} & {} & {} & {} & {} \\\\ \\hline".format(
        "\\textbf{Application}",
        "\\textbf{File}",
        "\\textbf{Mean}",
        "\\textbf{Standard deviation}",
        "\\textbf{Min}",
        "\\textbf{Max}"
    ))
    for app in apps:
        for i in range(len(labels)):
            print("{} & {} & {} & {} & {} & {} \\\\ \\hline".format(
                app,
                labels[i],
                stats[app]['mean'][i],
                stats[app]['stddev'][i],
                stats[app]['min'][i],
                stats[app]['max'][i]))
    print("\\end{tabular}")


apps = ['hermod', 'scp', 'sftp']
labels = ['10G', '1G', '500M', '10K', 'Hermod source code']
data_files = ['../output/hermod.json',
              '../output/scp.json', '../output/sftp.json']
stats = load_stats(apps, len(labels))
as_table(stats, apps, labels)

x = np.arange(len(labels))  # the label locations
width = 0.3  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(x - width, stats['hermod']['mean'], width, label='Hermod')
rects2 = ax.bar(x, stats['scp']['mean'], width, label='scp')
rects3 = ax.bar(x + width, stats['sftp']['mean'], width, label='sftp')

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
                    xy=(rect.get_x() + rect.get_width() / 4, height),
                    xytext=(0, 4),  # 3 points vertical offset
                    textcoords="offset points",
                    ha='center', va='bottom')


autolabel(rects1)
autolabel(rects2)
autolabel(rects3)

fig.tight_layout()

plt.show()
