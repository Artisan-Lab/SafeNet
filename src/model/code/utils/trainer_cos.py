import torch
import torch.nn.functional as F
import numpy as np
from sklearn.metrics import precision_score, recall_score, f1_score,accuracy_score
from torch.optim.lr_scheduler import CosineAnnealingLR
from transformers import AdamW,get_scheduler,get_linear_schedule_with_warmup
from tqdm import tqdm

from torch.cuda.amp import GradScaler
from torch.cuda.amp import autocast
from torch.nn import CrossEntropyLoss,BCEWithLogitsLoss,CosineEmbeddingLoss
from .baseloss import MultiFocalLoss

def get_optimizer_and_schedule(args, model, trainloader_shape):
    # 定义不需要权重衰减的参数
    no_decay = ['bias', 'LayerNorm.weight']

    # 冻结BERT模型的所有层，不进行反向传播和更新
    for param in model.model.parameters():
        param.requires_grad = False

    # 定义需要优化的参数，即任务特定的分类器层
    optimizer_grouped_parameters = [
        {'params': [p for n, p in model.named_parameters() if 'classifier' in n],
         'lr': args.learning_rate, 'weight_decay': 0.01}
    ]

    # 逐层解冻BERT模型，使用不同的学习率和权重衰减
    # lr = args.learning_rate
    # for idx, layer in enumerate(model.model.encoder.layer):
    #     layer.requires_grad = True
    #     lr *= args.lr_decay_rate
    #     weight_decay = args.weight_decay
    #     if idx == 0:
    #         weight_decay = 0.0
    #     optimizer_grouped_parameters += [
    #         {'params': [p for n, p in layer.named_parameters() if not any(nd in n for nd in no_decay)],
    #          'lr': lr, 'weight_decay': weight_decay},
    #         {'params': [p for n, p in layer.named_parameters() if any(nd in n for nd in no_decay)],
    #          'lr': lr, 'weight_decay': 0.0},
    #     ]

    # 定义优化器
    optimizer = AdamW(optimizer_grouped_parameters, lr=args.learning_rate, eps=1e-6)

    # 定义学习率调度器
    num_training_steps = int(trainloader_shape / args.train_batch_size * args.epoches)
    num_warmup_steps = int(num_training_steps * args.warmup_proportion)
    scheduler = get_linear_schedule_with_warmup(
        optimizer, num_warmup_steps=num_warmup_steps, num_training_steps=num_training_steps
    )

    return optimizer, scheduler
# 验证部分
def evaluation(args, model, data_loader, device,labels_name=None):
    model.eval()
    intent_preds = []
    intent_labels = []
    for batch in tqdm(data_loader):
        with torch.no_grad():
            input_ids_1 = batch['input_ids_1'].to(device)
            attention_mask_1 = batch['attention_mask_1'].to(device)
            input_ids_2 = batch['input_ids_2'].to(device)
            attention_mask_2 = batch['attention_mask_2'].to(device)
            labels = batch['label'].to(device)

            # forward + backward + optimize
            embed1, embed2 = model(input_ids_1, attention_mask_1, input_ids_2, attention_mask_2)
            intent_logits = torch.nn.functional.cosine_similarity(embed1, embed2)


            threshold = 0.9
            intent_preds_batch = [1 if x > threshold else -1 for x in intent_logits]
            # intent_logits = model(input_ids, attention_mask=attention_mask)

            intent_preds_batch = (intent_logits > threshold).cpu().numpy()
            # intent_preds_batch = torch.sigmoid(intent_logits).cpu().numpy()  # 计算预测结果 # 1,
            intent_labels_batch = labels.cpu().numpy()  # 获取标签

            intent_preds.append(intent_preds_batch)
            intent_labels.append(intent_labels_batch)
    y_pred = np.concatenate(intent_preds, axis=0)
    intent_labels = np.concatenate(intent_labels, axis=0)

    intent_acc = accuracy_score(y_true=intent_labels, y_pred=y_pred)  # 计算准确率
    precision = precision_score(y_true=intent_labels, y_pred=y_pred, average='macro')  # 计算精确率
    recall = recall_score(y_true=intent_labels, y_pred=y_pred, average='macro')  # 计算召回率
    f1 = f1_score(y_true=intent_labels, y_pred=y_pred, average='macro')  # 计算 F1 值
    print("Accuracy: {:.4f}, Precision: {:.4f}, Recall: {:.4f}, F1: {:.4f}".format(intent_acc, precision, recall, f1))
    return f1


# 训练阶段
def do_train(args, model, train_dataloader, dev_dataloader, device, intent2id,optimizer, scheduler):
    total_step = len(train_dataloader) * args.epoches
    intent_model_total_epochs = 0
    best_score = 0.0
    this_epoch_training_loss = 0
    stop_nums = 0
    iters_to_accumulate = args.grad_accumulate_nums
    # intent_loss_fct = CrossEntropyLoss()
    # intent_loss_fct = PriorMultiLabelSoftMarginLoss(prior=None, num_labels=len(intent2id))
    # intent_loss_fct = MultiFocalLoss(num_class=len(intent2id))
    # intent_loss_fct = MultiDSCLoss(alpha=0.3, smooth=1.0)
    # intent_loss_fct = BCEWithLogitsLoss()
    intent_loss_fct = CosineEmbeddingLoss()
    scaler = GradScaler()
    total_train_num = len(train_dataloader)
    # 训练
    print("train ...")
    for epoch in range(0, args.epoches):
        if stop_nums >= args.early_stopping:
            break
        model.train()
        for step, batch in enumerate(tqdm(train_dataloader), start=1):
            input_ids_1 = batch['input_ids_1'].to(device)
            attention_mask_1 = batch['attention_mask_1'].to(device)
            input_ids_2 = batch['input_ids_2'].to(device)
            attention_mask_2 = batch['attention_mask_2'].to(device)
            labels = batch['label'].to(device)
            optimizer.zero_grad()

            # forward + backward + optimize
            # outputs = model(input_ids_1, attention_mask_1, input_ids_2, attention_mask_2)
            embed1, embed2 = model(input_ids_1, attention_mask_1, input_ids_2, attention_mask_2)
            intent_loss = intent_loss_fct(embed1,embed2,labels.float())
            loss = intent_loss

            loss.backward()
            if (step + 1) % iters_to_accumulate == 0:
                optimizer.step()
                optimizer.zero_grad()
                scheduler.step()

            intent_model_total_epochs += 1

        if stop_nums >= args.early_stopping:
            break
        # 验证
        eval_intent_score = evaluation(args,model,dev_dataloader,device,labels_name=list(intent2id.keys()))
        # print("eval acc: %.5f" % eval_score)
        if best_score < eval_intent_score:
            stop_nums = 0
            print(r"【%.2f%%】 Intent F1-score update %.5f ---> %.5f " % ((intent_model_total_epochs/total_step),best_score, eval_intent_score))
            best_score = eval_intent_score
            # 保存模型
            torch.save(model.state_dict(),args.save_dir_curr)
            model.model.save_pretrained('{}/bert_wwm'.format('../user_data/checkpoint'))
        else:
            stop_nums = stop_nums + 1
            if stop_nums > int(args.early_stopping - 7):
                print("stop nums is {}".format(stop_nums))
    return best_score


