import os
import json
import random
import csv

def get_json_files(folder_path):
    json_files = []
    for file in os.listdir(folder_path):
        if file.endswith(".json"):
            json_files.append(os.path.join(folder_path, file))
    return json_files

def read_json(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return data

def generate_csv(json_files, output_file):
    with open(output_file, 'w', newline='', encoding='utf-8') as csvfile:
        csv_writer = csv.writer(csvfile)
        csv_writer.writerow([' ', 'text1', 'text2', ' '])
        combinations = [(file1, file2) for file1 in json_files for file2 in json_files if file1 != file2]

        random.shuffle(combinations)
        fold_size = len(combinations) // 3

        for i, (file1, file2) in enumerate(combinations):
            json_data1 = read_json(file1)
            json_data2 = read_json(file2)

            match = 1 if os.path.basename(file1)[0] == os.path.basename(file2)[0] else -1

            csv_writer.writerow([i + 1, json.dumps(json_data1), json.dumps(json_data2), match])

            if i + 1 == fold_size or i + 1 == fold_size * 2:
                csvfile.close()
                if i + 1 == fold_size:
                    output_file = f"test{i // fold_size + 1}.csv"
                else:
                    output_file = f"train{i // fold_size}.csv"
                csvfile = open(output_file, 'w', newline='', encoding='utf-8')
                csv_writer = csv.writer(csvfile)
                csv_writer.writerow([' ', 'text1', 'text2', ' '])

folder_path = ''

json_files = get_json_files(folder_path)

output_csv = 'output.csv'
generate_csv(json_files, output_csv)



# import os
# import json
# import random
# import csv
# from itertools import combinations
# from sklearn.model_selection import KFold

# folder_path = ' '

# json_files = [file for file in os.listdir(folder_path) if file.endswith('.json')]

# # 生成两两组合的列表
# combinations_list = list(combinations(json_files, 2))

# random.shuffle(combinations_list)

# data = []

# # 两两组合
# for idx, (file1, file2) in enumerate(combinations_list, start=1):
#     with open(os.path.join(folder_path, file1), 'r') as f1, open(os.path.join(folder_path, file2), 'r') as f2:
#         json_content_1 = json.load(f1)
#         json_content_2 = json.load(f2)

#         label = 1 if file1[0] == file2[0] else -1

#         data.append([idx, json_content_1, json_content_2, label])
      
# kf = KFold(n_splits=3, shuffle=True)
# fold_idx = 1

# for train_indices, test_indices in kf.split(data):
#     train_data = [data[i] for i in train_indices]
#     test_data = [data[i] for i in test_indices]

#     # 写入训练集和测试集的 CSV 文件
#     with open(f"train{fold_idx}.csv", 'w', newline='') as train_file:
#         writer = csv.writer(train_file)
#         writer.writerow([' ', 'text1', 'text2', ' '])
#         writer.writerows(train_data)

#     with open(f"test{fold_idx}.csv", 'w', newline='') as test_file:
#         writer = csv.writer(test_file)
#         writer.writerow([' ', 'text1', 'text2', ' '])
#         writer.writerows(test_data)

#     fold_idx += 1


