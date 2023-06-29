import torch
from transformers import AutoModel, AutoTokenizer
from utils.model_utils_bert import XFBert
import json
from tqdm import tqdm
import os
import heapq
import torch
import torch.nn.functional as F

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

                    query_cos = model(input_ids_1, attention_mask_1, input_ids_2, attention_mask_2)
                    tensor1 = query_cos[0]
                    tensor2 = query_cos[1]
                    cosine_similarity = F.cosine_similarity(tensor1, tensor2, dim=1)
                    Similarity.append(cosine_similarity.item())
            
                # 使用堆数据结构获取相似度最高的3个值及其对应的索引
                top_k_similarities = heapq.nlargest(3, Similarity)
                top_k_indices = [Similarity.index(similarity) for similarity in top_k_similarities]

                top_k_results = []
                for index in top_k_indices:
                    most_similar_text_id = category_vectors_base[sample['category']][index][1]
                    most_similar_category = category_vectors_base[sample['category']][index][2]
                    similarity_value = Similarity[index]

                    top_k_results.append({
                        'predict': most_similar_text_id,
                        'category': most_similar_category,
                        'Similarity_value': similarity_value
                    })

                result = {
                    'id': sample['id'],
                    'text': sample['text'],
                    'category': sample['category'],
                    'similarity': sample['similarity'],
                    'top_k_results': top_k_results
                }
                results.append(result)

            f.write(json.dumps(results, ensure_ascii=False, indent=4) + '\n')
                



