import time
import torch
import random
import numpy as np
from datetime import timedelta

def set_seed(seed):
    np.random.seed(seed)
    random.seed(seed)
    torch.manual_seed(seed) #CPU随机种子确定
    torch.cuda.manual_seed(seed) #GPU随机种子确定
    torch.cuda.manual_seed_all(seed) #所有的GPU设置种子

def get_time_dif(start_time):
    """
    获取已经使用的时间
    :param start_time:
    :return:
    """
    end_time = time.time()
    time_dif = end_time - start_time
    return timedelta(seconds=int(round(time_dif)))

# 给出的标签均为文本，创建label_map构建映射关系
def get_label_map(label_list):
    id2label = dict([(idx, label) for idx, label in enumerate(label_list)])
    label2id = dict([(label, idx) for idx, label in enumerate(label_list)])
    return id2label, label2id

def get_device():
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    return device

