import os

import numpy as np
import pandas as pd
import torch
import torch.nn as nn
from transformers import  BertModel,BertConfig,RobertaConfig,RobertaModel

class Mdrop(nn.Module):
    def __init__(self):
        super(Mdrop,self).__init__()
        self.dropout_0 = nn.Dropout(p=0.0)
        self.dropout_1 = nn.Dropout(p=0.1)
        self.dropout_2 = nn.Dropout(p=0.2)
        self.dropout_3 = nn.Dropout(p=0.3)
    def forward(self,x):
        output_0 = self.dropout_0(x)
        output_1 = self.dropout_1(x)
        output_2 = self.dropout_2(x)
        output_3 = self.dropout_3(x)
        return [output_0,output_1,output_2,output_3]

class XFBert(nn.Module):
    def __init__(self, MODEL_NAME, intent_dim, dropout=None, n_ambda=0., enable_mdrop=False):
        super(XFBert, self).__init__()
        self.enable_mdrop = enable_mdrop
        self.model = BertModel.from_pretrained(MODEL_NAME).requires_grad_(False)
        if n_ambda > 0.:
            print(n_ambda)
            for name, para in self.model.named_parameters():
                self.model.state_dict()[name][:] += (torch.rand(para.size()) - 0.5) * n_ambda * torch.std(para)
        self.config = BertConfig.from_pretrained(MODEL_NAME)
        self.intent_num_labels = intent_dim

        if self.enable_mdrop:
            self.dropout = Mdrop()
        else:
            self.dropout = nn.Dropout(dropout if dropout is not None else self.config.hidden_dropout_prob)
        self.fc_layers = nn.Sequential(
            nn.Linear(self.config.hidden_size, 128),
            nn.Linear(128, 32),
        )
        self.intent_classifier = nn.Linear(self.config.hidden_size*2, self.intent_num_labels)
        


    def forward(self, input_ids_1, attention_mask_1, input_ids_2, attention_mask_2):
        outputs_1 = self.model(input_ids_1, attention_mask=attention_mask_1)
        outputs_2 = self.model(input_ids_2, attention_mask=attention_mask_2)

        pooled_output_1 = outputs_1.pooler_output
        pooled_output_2 = outputs_2.pooler_output

        pooled_output_1 = self.fc_layers(pooled_output_1)
        pooled_output_2 = self.fc_layers(pooled_output_2)

        pooled_output_1 = pooled_output_1 / pooled_output_1.norm(dim=1, keepdim=True) # bs,32
        pooled_output_2 = pooled_output_2 / pooled_output_2.norm(dim=1, keepdim=True) # bs,32


        # cos_value = torch.nn.functional.cosine_similarity(pooled_output_1, pooled_output_2)

        return pooled_output_1, pooled_output_2
