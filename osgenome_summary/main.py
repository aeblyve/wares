import openai
import sys
import openpyxl

openai.api_key = "<CHANGEME>"

def slice_between_substrings(s, start_sub, end_sub):
    start = s.find(start_sub)
    end = s.find(end_sub)
    if start != -1 and end != -1 and start < end: # ensure both substrings are in s and start_sub is before end_sub
        return s[start + len(start_sub) : end]
    else:
        return None # or some other value indicating that the slice couldn't be made

input_file = sys.argv[1]

workbook = openpyxl.load_workbook(input_file)
worksheet = workbook.active

phenotypes = []

for row in worksheet.iter_rows(min_col=5, max_col=5):
    for cell in row:
        sliced_value = slice_between_substrings(cell.value, "<b>", "</b>")
        if sliced_value is not None:
            phenotypes.append(sliced_value)

phenotype_report = "\n".join(phenotypes)

response = openai.ChatCompletion.create(
  model="gpt-4",
  messages=[
        {"role": "system", "content": "I am going to send you a report of phenotypes as derived from a SNP analysis. You job is to summarize the report into one paragraph. Focus on the genetic traits, with little exposition."},
        {"role": "user", "content": phenotype_report},
    ]
)

print(response['choices'][0]['message']['content'])
