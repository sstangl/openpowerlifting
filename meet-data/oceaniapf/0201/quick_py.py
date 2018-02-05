import csv

def IsInt(check_str):
	try:
		value = int(check_str)
		return True
	except ValueError:
		return False



file = open("entries.csv","r")
reader =csv.reader(file)


meet = list(reader)
file.close()

for ii in range(0,len(meet)):
	for jj in range(0,len(meet[ii])):
		if len(meet[ii][jj])>1:
			if IsInt(meet[ii][jj][0]) and meet[ii][jj][-1]=='F':
				meet[ii][jj]='-'+meet[ii][jj][0:-1]


	o_file = open("entries.csv",'w', newline='')
	writer =csv.writer(o_file)
	writer.writerows(meet)
	o_file.close()