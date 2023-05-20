import csv
import sys
import logging
import requests
from bs4 import BeautifulSoup

logging.basicConfig(filename="snp_reader.log",
                    filemode='a',
                    format='%(asctime)s,%(msecs)d %(name)s %(levelname)s %(message)s',
                    datefmt='%H:%M:%S',
                    level=logging.DEBUG)

input_file = sys.argv[1]
output_file = sys.argv[2]

with open(input_file) as tsvfile:
    with open(output_file, "w") as outfile:
        reader = csv.reader(tsvfile, delimiter='\t')
        for row in reader:
            if row[0].startswith("#"):
                continue
            logging.info(f"Importing row: {row}")
            snp = row[0]
            chromosome = row[1]
            position = row[2]
            genotype = row[3]

            url = f"https://bots.snpedia.com/index.php/{snp}"
            response = requests.get(url)

            soup = BeautifulSoup(response.text, "html.parser")
            content = soup.find("div", {"id": "mw-context-text"})

            try:
                output_file.write(content.get_text())
            except:
                logging.info(f"No page for snp {snp}.")
