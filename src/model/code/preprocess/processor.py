import torch
from torch.utils.data import Dataset, DataLoader

class SentencePairDataset(Dataset):
    def __init__(self, data, tokenizer, max_seq_len):
        self.data = data
        self.tokenizer = tokenizer
        self.max_seq_len = max_seq_len

    def __len__(self):
        return len(self.data)

    def __getitem__(self, i):
        text1 = self.data.iloc[i]['text1']
        text2 = self.data.iloc[i]['text2']
        label = self.data.iloc[i]['label']

        # tokenize the sentences
        tokenized_input1 = self.tokenizer(text1, truncation=True, max_length=self.max_seq_len,pad_to_max_length=True )
        tokenized_input2 = self.tokenizer(text2, truncation=True, max_length=self.max_seq_len, pad_to_max_length=True)

        input_ids_1 = torch.tensor(tokenized_input1['input_ids'])
        attention_mask_1 = torch.tensor(tokenized_input1['attention_mask'])
        input_ids_2 = torch.tensor(tokenized_input2['input_ids'])
        attention_mask_2 = torch.tensor(tokenized_input2['attention_mask'])

        return {
            'input_ids_1': input_ids_1,
            'attention_mask_1': attention_mask_1,
            'input_ids_2': input_ids_2,
            'attention_mask_2': attention_mask_2,
            'label': label
        }