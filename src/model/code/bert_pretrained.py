import torch
import torch.nn as nn
from torch.utils.data import Dataset
import transformers
from transformers import BertTokenizer, BertModel, BertConfig
import operator
import json

import os

import random
from torch.utils.data.dataloader import DataLoader
from torch.utils.data import RandomSampler
from torch.optim import lr_scheduler
from tqdm import tqdm
from typing import Optional, Union, Iterable
import logging
import time
from torch.nn import CrossEntropyLoss
from utils.utils import Config, Logger


import torch.nn.functional as F
from sklearn.metrics.pairwise import euclidean_distances

import numpy as np
from sklearn.metrics.pairwise import pairwise_distances

# import Levenshtein




file_name = {
    "get_unchecked": "get_unchecked_unsafe",
    "get_unchecked_mut": "get_unchecked_mut",
    "array_assume_init": "array_assume_init",
    "assume_init_26": "assume_init_unsafe",
    "from_raw_1": "from_raw_unsafe",
    "from_raw_parts": "from_raw_parts_unsafe",
    "from_raw_parts_mut": "from_raw_parts_mut",
    "from_utf8_unchecked": "from_utf8_unchecked",
    "from_u32_unchecked": "from_u32_unchecked",
    "new_unchecked": "new_unchecked",
    "mem_zeroed": "mem_zeroed",
    "new_unchecked": "new_unchecked",
    "offset_1": "offset_unsafe",
    "offset_from": "offset_from",
    "pointer_add": "pointer_add",
    "raw_pointer_deref": "raw_pointer_deref",
    "set_len": "set_len",
    "transmute": "transmute",
    "write": "write",
}


def load_file(path):
    assert os.path.exists(path), '文件不存在'
    data = {}
    for filename in os.listdir(path):
        if filename.endswith('.json'):
            with open(path+'/'+filename, 'r', encoding='utf-8') as file:
                content = eval(file.read())

                data.update({filename: " ".join(content['Preorder'])})
    return data


class SmiliarDataset(Dataset):
    def __init__(self, data, tokenizer):

        self.tokenizer = tokenizer
        input_ids = self.tokenizer(data, add_special_tokens=False)['input_ids']

        self.vocab, self.max_length = self.get_vocab(input_ids)

        self.masked_token_id = self.tokenizer.mask_token_id
        self.cls_token_id = self.tokenizer.cls_token_id
        self.sep_token_id = self.tokenizer.sep_token_id

        self.data, self.attn_mask, self.labels = self.preprocess(input_ids)

    def preprocess(self, data):

        samples = torch.zeros(len(data), self.max_length+2, dtype=torch.long)
        labels = torch.zeros(len(data), self.max_length+2, dtype=torch.long)
        attn_mask = torch.zeros(
            (len(data), self.max_length+2), dtype=torch.long)
        for i, tokens in enumerate(data):
            label = self.add_special_token(tokens)
            labels[i, :len(label)] = torch.tensor(label)

            sample = self.mask(tokens)

            sample = self.add_special_token(sample)
            samples[i, :len(sample)] = torch.tensor(sample)

            attn_mask[i, :len(sample)] = 1

        return samples, attn_mask, labels

    def mask(self, tokens):

        for random_index in random.sample(range(len(tokens)), int(len(tokens)*0.15)):
            if random.random() < 0.8:
                masked_tokens = self.masked_token_id
            else:
                if random.random() < 0.5:
                    masked_tokens = tokens[random_index]
                else:
                    masked_tokens = random.choice(self.vocab)
            tokens[random_index] = masked_tokens
        return tokens

    def add_special_token(self, tokens):
        return [self.cls_token_id]+tokens+[self.sep_token_id]

    def get_vocab(self, input_ids):
        vocab = set()
        max_length = 0
        for tokens in input_ids:
            if max_length < len(tokens):
                max_length = len(tokens)
            vocab.update(set(tokens))
        return list(vocab), max_length

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        X = {
            'input_ids': self.data[idx],
            'token_type_ids': torch.zeros_like(self.data[idx], dtype=torch.long),
            'attention_mask': self.attn_mask[idx]
        }
        y = self.labels[idx]
        return X, y


class MyModel(nn.Module):
    def __init__(self, bert_path, vocab_size, seq_len):
        super().__init__()
        self.bert = BertModel.from_pretrained(bert_path)
        self.fn = nn.Linear(seq_len, vocab_size)
        self.ls_fn = CrossEntropyLoss()
        self.vocab_size = vocab_size

    def forward(self, x, token_type_ids, attention_mask, y):
        last_hidden_state = self.bert(x, token_type_ids, attention_mask)[
            'last_hidden_state']
        logits = self.fn(last_hidden_state)

        loss = self.ls_fn(logits.view(-1, self.vocab_size), y.view(-1))
        return loss

    def get_train_params(self):
        params = []
        for key in self.bert.state_dict().keys():
            if 'encode.layer.11' in key:
                params.append(self.bert.state_dict()[key])
        for param in self.fn.parameters():
            params.append(param)
        return [{'params': params}]
    

    def search_similar_sample(self, tokenizer, test_data, targets):
        result = {}
        for key in test_data.keys():
            area = ""
            for name in file_name.keys():
                if name in key:
                    area = file_name[name]

            seq = tokenizer(test_data[key], return_tensors="pt").to('cuda')
            seq = self.bert(**seq)['pooler_output']
            max_value = 0
            max_key = ""
            for target_key in targets.keys():
                if area in target_key:
                    target = tokenizer(targets[target_key], return_tensors="pt").to('cuda')
                    target = self.bert(**target)['pooler_output']
                
                    # use Jaccard similarity instead of cosine similarity
                    seq_bin = (seq > 0).cpu().numpy().reshape(1, -1)
                    target_bin = (target > 0).cpu().numpy().reshape(1, -1)
                    sim = 1 - pairwise_distances(seq_bin, target_bin, metric='jaccard')[0,0]
                
                    if sim > max_value:
                        max_key = target_key
                        max_value = sim
            result[key] = max_key
    
        with open('result.json', 'w', encoding='utf-8') as f:
            json.dump(result, f, ensure_ascii=False, indent=4)
    
        return result
        
  

OPTIMIZER = {
    'Adam': torch.optim.Adam,
    'AdamW': torch.optim.AdamW,
    'SGD': torch.optim.SGD,
    'AdaGrad': torch.optim.Adagrad
}


class Trainer:

    def __init__(self, model, config, train_dataset):

        self.config = config
        self.train_dataset = train_dataset

        if config.device == 'auto':
            self.device = 'cuda' if torch.cuda.is_available() else 'cpu'
        else:
            self.device = config.device

        self.model = model.to(self.device)
        logging.info("running on device "+self.device)

        self.train_params = self.model.get_train_params() if hasattr(
            model, 'get_train_params') else self.model.parameters()

        assert config.optim in OPTIMIZER, "The optimizer is not in OPTIMIZER,please select the following optimizer:" + \
            ", ".join(list(OPTIMIZER.keys()))

        assert isinstance(
            config.optim_args, dict), "The type of the optim_args must be dict, but it is "+type(config.optim_args).__name__

        self.optim_args = config.optim_args
        self.set_optimizer(OPTIMIZER[config.optim],
                           self.train_params,
                           self.config.lr,
                           **self.optim_args
                           )

    def set_optimizer(self,
                      optimizer: Optional[torch.optim.Optimizer],
                      params: Optional[Union[torch.Tensor,
                                             dict, list, Iterable]] = None,
                      lr: Optional[float] = None,
                      **kwargs
                      ) -> None:

        if params == None:
            params = self.train_params

        self.optimizer = optimizer(params, lr=lr, **kwargs)

        logging.info("Optimizer:"+self.optimizer.__class__.__name__)

    def run(self):

        assert self.optimizer != None, "The optimizer is none,please set a optimizer"

        #scheduler = lr_scheduler.MultiStepLR(self.optimizer, [2,4], gamma=0.5, last_epoch=-1)

        train_loader = DataLoader(
            self.train_dataset,
            sampler=RandomSampler(self.train_dataset,
                                  replacement=True, num_samples=int(1e10)),
            shuffle=False,
            pin_memory=True,
            batch_size=self.config.batch_size,
            num_workers=self.config.num_workers,
        )

        self.model.train()
        data_iter = iter(train_loader)
        for e in range(self.config.epochs):
            iter_num = 0
            start_time = time.time()
            loss = 0
            with tqdm(total=self.config.max_items,) as t:
                while True:
                    try:
                        batch = next(data_iter)
                    except StopIteration:
                        data_iter = iter(train_loader)
                        batch = next(data_iter)
                    X, y = batch
                    x = X['input_ids'].to(self.device)
                    token_type_ids = X['token_type_ids'].to(self.device)
                    attention_mask = X['attention_mask'].to(self.device)
                    y = y.to(self.device)
                    loss = self.model(
                        x, token_type_ids=token_type_ids, attention_mask=attention_mask, y=y)

                    self.model.zero_grad(set_to_none=True)
                    loss.backward()
                    torch.nn.utils.clip_grad_norm_(
                        self.model.parameters(), 1.0)
                    self.optimizer.step()

                    iter_num += 1
                    t.update(1)
                    t.postfix = ' - loss: '+str(loss.item())

                    if self.config.max_items is not None and iter_num > self.config.max_items:
                        break
            end_time = time.time()

            logging.info(f"epoch: {e+1} -- time: {end_time-start_time:.2f}s")


if __name__ == '__main__':
    train_data = load_file("../data/pretrained/train")
    # print(list(train_data.values()))
    data_train = list(train_data.values())
    train_list = []
    for i in range(len(data_train)):
        for j in range(0, len(data_train[i]), 512):
            train_list.append(data_train[i][j:j + 512])
    log = Logger()
    conf = Config()
    conf.update({
        'bert_path': r'/home/rose/code/similar/bert-base-uncased',
        'device': 'auto',
        'epochs': 20 ,
        'batch_size': 16,
        'num_workers': 0,
        'optim': 'AdamW',
        'lr': 0.001,
        'optim_args': {'betas': [0.9, 0.95]},
        "vocab_size": 30522,
        "hidden_size": 768,
    })
    tokenizer = BertTokenizer.from_pretrained(conf.bert_path)
    tokenizer.save_vocabulary('{}/bert_wwm_mlm'.format('../user_data/checkpoint'))

    model = MyModel(conf.bert_path, conf.vocab_size, conf.hidden_size)
    model.to('cuda')
    smiliar_dataset = SmiliarDataset(
        train_list, tokenizer=tokenizer)
    conf.update({'max_items': len(smiliar_dataset)})
    print(smiliar_dataset[0][0]['input_ids'].shape)
    trainer = Trainer(model, conf, smiliar_dataset)
    trainer.run()
    model.bert.save_pretrained('{}/bert_wwm_mlm'.format('../user_data/checkpoint'))

    # 获取BERT模型的配置信息
    config = BertConfig.from_pretrained(conf.bert_path)
    # 保存配置信息到文件
    config.save_pretrained('{}/bert_wwm_mlm'.format('../user_data/checkpoint'))


        
