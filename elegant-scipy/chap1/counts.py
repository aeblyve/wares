import bz2
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import itertools as it
from collections import defaultdict

from scipy import stats

# import TCGA melanoma data
filename = './counts.txt.bz2'
with bz2.open(filename, 'rt') as f:
    data_table = pd.read_csv(f, index_col=0)

print(data_table.iloc[:5, :5])

samples = list(data_table.columns)

filename = './genes.csv'
with open(filename, 'rt') as f:
    gene_info = pd.read_csv(f, index_col=0)
print(gene_info.iloc[:5, :])

print("Genes in data_table: ", data_table.shape[0])
print("Genes in gene_info: ", gene_info.shape[0])

matched_index = pd.Index.intersection(data_table.index, gene_info.index)

counts = np.asarray(data_table.loc[matched_index], dtype=int)
gene_names = np.array(matched_index)

print(f'{counts.shape[0]} genes measured in {counts.shape[1]} individuals.')

gene_lengths = np.asarray(gene_info.loc[matched_index]['GeneLength'], dtype=int)

print(counts.shape)
print(gene_lengths.shape)
plt.style.use('./elegant.mplstyle')

total_counts = np.sum(counts, axis=0)

density = stats.kde.gaussian_kde(total_counts)

x = np.arange(min(total_counts), max(total_counts), 10000)

fig, ax = plt.subplots()
ax.plot(x, density(x))
ax.set_xlabel("Total counts per individual")
ax.set_ylabel("Density")
plt.show()


print(f'Count statistics:\n  min:  {np.min(total_counts)}'
      f'\n  mean: {np.mean(total_counts)}'
      f'\n  max:  {np.max(total_counts)}')

np.random.seed(seed=7)
samples_index = np.random.choice(range(counts.shape[1]), size=70, replace=False)
counts_subset = counts[:, samples_index]

def reduce_xaxis_labels(ax, factor):
    plt.setp(ax.xaxis.get_ticklabels(), visible=False)
    for label in ax.xaxis.get_ticklabels()[factor-1::factor]:
        label.set_visible(True)


# Normalize by individual's "library size"

counts_lib_norm = counts / total_counts  * 1e6
counts_subset_lib_norm = counts_lib_norm[:, samples_index]


fig, ax = plt.subplots(figsize=(4.8, 2.4))
with plt.style.context('./thinner.mplstyle'):
    ax.boxplot(np.log(counts_subset + 1))
    ax.set_xlabel("Individuals")
    ax.set_ylabel("Gene expression counts")
    reduce_xaxis_labels(ax, 5)
    plt.show()


def class_boxplot(data, classes, colors=None, **kwargs):
    all_classes = sorted(set(classes))
    colors = plt.rcParams['axes.prop_cycle'].by_key()['color']
    class2color = dict(zip(all_classes, it.cycle(colors)))

    # map classes to data vectors
    # other classes get an empty list at that position for offset
    class2data = defaultdict(list)
    for distrib, cls in zip(data, classes):
        for c in all_classes:
            class2data[c].append([])
        class2data[cls][-1] = distrib

    # then, do each boxplot in turn with the appropriate color
    fig, ax = plt.subplots()
    lines = []
    for cls in all_classes:
        # set color for all elements of the boxplot
        for key in ['boxprops', 'whiskerprops', 'flierprops']:
            kwargs.setdefault(key, {}).update(color=class2color[cls])
        # draw the boxplot
        box = ax.boxplot(class2data[cls], **kwargs)
        lines.append(box['whiskers'][0])
    ax.legend(lines, all_classes)
    return ax

log_counts_3 = list(np.log(counts.T[:3] + 1))
log_ncounts_3 = list(np.log(counts_lib_norm.T[:3] + 1))
ax = class_boxplot(log_counts_3 + log_ncounts_3,
                   ['raw counts'] * 3 + ['normalized by library size'] * 3,
                   labels=[1, 2, 3, 1, 2, 3])
ax.set_xlabel('sample number')
ax.set_ylabel('log gene expression counts');
plt.show()

def binned_boxplot(x, y, *,  # check out this Python 3 exclusive! (*see tip box)
                   xlabel='gene length (log scale)',
                   ylabel='average log counts'):
    """Plot the distribution of `y` dependent on `x` using many boxplots.

    Note: all inputs are expected to be log-scaled.

    Parameters
    ----------
    x: 1D array of float
        Independent variable values.
    y: 1D array of float
        Dependent variable values.
    """
    # Define bins of `x` depending on density of observations
    x_hist, x_bins = np.histogram(x, bins='auto')

    # Use `np.digitize` to number the bins
    # Discard the last bin edge because it breaks the right-open assumption
    # of `digitize`. The max observation correctly goes into the last bin.
    x_bin_idxs = np.digitize(x, x_bins[:-1])

    # Use those indices to create a list of arrays, each containing the `y`
    # values corresponding to `x`s in that bin. This is the input format
    # expected by `plt.boxplot`
    binned_y = [y[x_bin_idxs == i]
                for i in range(np.max(x_bin_idxs))]
    fig, ax = plt.subplots(figsize=(4.8,1))

    # Make the x-axis labels using the bin centers
    x_bin_centers = (x_bins[1:] + x_bins[:-1]) / 2
    x_ticklabels = np.round(np.exp(x_bin_centers)).astype(int)

    # make the boxplot
    ax.boxplot(binned_y, labels=x_ticklabels)

    # show only every 10th label to prevent crowding on x-axis
    reduce_xaxis_labels(ax, 10)

    # Adjust the axis names
    ax.set_xlabel(xlabel)
    ax.set_ylabel(ylabel)
    return ax

log_counts = np.log(counts_lib_norm + 1)

mean_log_counts = np.mean(log_counts, axis=1)
log_gene_lengths = np.log(gene_lengths)

with plt.style.context('./thinner.mplstyle'):
    binned_boxplot(x=log_gene_lengths, y=mean_log_counts)
    plt.show()

"""
    One of the simplest normalization methods for RNAseq data is RPKM: reads per kilobase transcript per million reads. RPKM puts together the ideas of normalizing by sample and by gene. When we calculate RPKM, we are normalizing for both the library size (the sum of each column) and the gene length.
"""
def rpkm(counts, lengths):
    """Calculate reads per kilobase transcript per million reads.

    RPKM = (10^9 * C) / (N * L)

    Where:
    C = Number of reads mapped to a gene
    N = Total mapped reads in the experiment
    L = Exon length in base pairs for a gene

    Parameters
    ----------
    counts: array, shape (N_genes, N_samples)
        RNAseq (or similar) count data where columns are individual samples
        and rows are genes.
    lengths: array, shape (N_genes,)
        Gene lengths in base pairs in the same order
        as the rows in counts.

    Returns
    -------
    normed : array, shape (N_genes, N_samples)
        The RPKM normalized counts matrix.
    """
    C = counts.astype(float)  # use float to avoid overflow with `1e9 * C`
    N = np.sum(C, axis=0)  # sum each column to get total reads per sample
    L = lengths

    normed = 1e9 * C / (N[np.newaxis, :] * L[:, np.newaxis]) # appends new dimensions with values 1

    return(normed)


counts_rpkm = rpkm(counts, gene_lengths)
log_counts = np.log(counts_rpkm + 1)
mean_log_counts = np.mean(log_counts, axis=1)
log_gene_lengths = np.log(gene_lengths)
lim = binned_boxplot(x=log_gene_lengths, y=mean_log_counts).get_ylim()

with plt.style.context('.//thinner.mplstyle'):
    rpkm_ax = binned_boxplot(x=log_gene_lengths, y=mean_log_counts)
    rpkm_ax.set_ylim(lim)
    plt.show()






