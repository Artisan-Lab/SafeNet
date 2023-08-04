import torch
from transformers import AutoModel, AutoTokenizer
from utils.model_utils_bert import XFBert
import json
from tqdm import tqdm
import os
import heapq

def encode(document: str) -> torch.Tensor:
    tokens = tokenizer(document, return_tensors='pt', max_length=256, truncation=True)
    vector = model(**tokens)[0].detach().squeeze()
    return torch.mean(vector, dim=0)


if __name__ == "__main__":

    knowledgebase = json.load(open('../data/baaaa/knowledgebase.json', 'r'))

    tokenizer = AutoTokenizer.from_pretrained(r'../user_data/checkpoint/bert_wwm')
    model = XFBert(r'../user_data/checkpoint/bert_wwm', intent_dim=2).requires_grad_(False)
    model.load_state_dict(torch.load(os.path.join("../user_data/checkpoint", 'parameter_{}.pkl'.format('bert-base-wwm'))))

    # 冻结BERT模型的所有参数
    for param in model.model.parameters():
        param.requires_grad = False

    model.eval()
    with torch.no_grad():

        category_vectors_base = {}
        for sample in knowledgebase:
            if sample['category'] not in category_vectors_base:
                category_vectors_base[sample['category']] = []
            category_vectors_base[sample['category']].append((sample['text'], sample['id'], sample['category']))

        test = json.load(open('../data/baaaa/test.json', 'r'))

        with open('../user_data/results/result.json', 'w', encoding='utf-8-sig') as f:
            results = []
            for idx,sample in enumerate(tqdm(test)):

                Similarity = []
                for base in category_vectors_base[sample['category']]:
                    tokenized_input1 = tokenizer(sample['text'], truncation=True, max_length=100,
                                                      pad_to_max_length=True)
                    tokenized_input2 = tokenizer(base[0], truncation=True, max_length=100,
                                                      pad_to_max_length=True)

                    input_ids_1 = torch.tensor(tokenized_input1['input_ids']).unsqueeze(0)
                    attention_mask_1 = torch.tensor(tokenized_input1['attention_mask']).unsqueeze(0)
                    input_ids_2 = torch.tensor(tokenized_input2['input_ids']).unsqueeze(0)
                    attention_mask_2 = torch.tensor(tokenized_input2['attention_mask']).unsqueeze(0)
                    embed1, embed2 = model(input_ids_1, attention_mask_1, input_ids_2, attention_mask_2)
                    cosine_similarity = torch.nn.functional.cosine_similarity(embed1, embed2)
                    Similarity.append(cosine_similarity.item())

                max_similarity_index = Similarity.index(max(Similarity))
                most_similar_text_id = category_vectors_base[sample['category']][max_similarity_index][1]
                most_similar_category = category_vectors_base[sample['category']][max_similarity_index][2]

                result = {}
                result['id'] = sample['id']
                result['text'] = sample['text']
                result['category'] = sample['category']
                result['similarity'] = sample['similarity']
                result['predict'] = most_similar_text_id
                result['Similarity_value'] = max(Similarity)
                results.append(result)
            f.write(json.dumps(results, ensure_ascii=False, indent=4) + '\n')


/* 
        // top3
        with open('../user_data/results/result.json', 'w', encoding='utf-8-sig') as f:
            results = []
            for idx, sample in enumerate(tqdm(test)):
                Similarity = []
                indices = []
                for base in category_vectors_base[sample['category']]:
                    tokenized_input1 = tokenizer(sample['text'], truncation=True, max_length=100, pad_to_max_length=True)
                    tokenized_input2 = tokenizer(base[0], truncation=True, max_length=100, pad_to_max_length=True)

                    input_ids_1 = torch.tensor(tokenized_input1['input_ids']).unsqueeze(0)
                    attention_mask_1 = torch.tensor(tokenized_input1['attention_mask']).unsqueeze(0)
                    input_ids_2 = torch.tensor(tokenized_input2['input_ids']).unsqueeze(0)
                    attention_mask_2 = torch.tensor(tokenized_input2['attention_mask']).unsqueeze(0)

                    embed1, embed2 = model(input_ids_1, attention_mask_1, input_ids_2, attention_mask_2)

                    lp_distance = torch.norm(embed1 - embed2, p=p, dim=1)
                    similarity = 1 / (1 + lp_distance)

                    Similarity.append(similarity.item())
                    indices.append(base[1])  # 添加相似度对应的index值

                # 使用堆来获取相似度最大的前三个结果
                top_similarities = heapq.nlargest(1, zip(Similarity, indices))

                result = {}
                result['id'] = sample['id']
                result['text'] = sample['text']
                result['category'] = sample['category']
                result['similarity'] = sample['similarity']
                result['predict'] = [x[1] for x in top_similarities]
                result['Similarity_value'] = [x[0] for x in top_similarities]
                results.append(result)
            f.write(json.dumps(results, ensure_ascii=False, indent=4) + '\n')

*/



