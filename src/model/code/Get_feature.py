import pandas as pd
import numpy as np
import warnings

warnings.filterwarnings("ignore")

from transformers import AutoTokenizer, AutoModel
import torch
import torch.nn as nn
import torch.nn.functional as F
import pandas as pd

# 加载BERT模型和分词器
tokenizer = AutoTokenizer.from_pretrained("../user_data/checkpoint/bert_wwm_mlm")
model = AutoModel.from_pretrained("../user_data/checkpoint/bert_wwm_mlm")
# model.load_state_dict(torch.load('model.pt'))


# 定义bert编码函数
def bert2bedding(s1):
    # 对句子进行分词，并添加特殊标记
    inputs = tokenizer(s1, return_tensors='pt', padding=True, truncation=True)

    # 将输入传递给BERT模型，并获取输出
    with torch.no_grad():
        outputs = model(**inputs)
        embeddings = outputs.last_hidden_state[:, 0, :].cpu().numpy().reshape(1, 768)

    return embeddings


if __name__ == '__main__':
    train_data = pd.read_csv("./data/train/train.csv", index_col=None)
    # 特征文件的保存
    features = []

    # 统计样本的数量
    samples_count = train_data.shape[0]

    for i in range(0, samples_count, 10):
        t1 = train_data.iloc[i][1]
        t2 = train_data.iloc[i][2]
        Label = np.zeros((1, 1))
        Label[0][0] = train_data.iloc[i][3]

        Es1 = bert2bedding(t1)
        Es2 = bert2bedding(t2)


        # feature 的汇总与保存
        feature = np.hstack((Es1, Es2, Label))
        feature = np.squeeze(feature)
        features.append(feature)

    # 数据特征的保存
    features = pd.DataFrame(np.array(features))
    features.to_csv("./feature.csv", index=False)

    print("特征数据提取完成，已保存!")
