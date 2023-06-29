import os
import json


class TreeNode:
    def __init__(self, value):
        self.value = value
        self.children = []

    def add_child(self, child):
        self.children.append(child)

    def preorder_traversal(self):
        result = [self.value]
        for child in self.children:
            result.extend(child.preorder_traversal())
        return result


def complete_tree(node):
    if not node.children:
        return

    for i in range(len(node.children)):
        child = node.children[i]
        if child.value == '()':
            child.add_child(TreeNode('0'))
        complete_tree(child)

        # Update the value of the current node with the number of children
        node.value = f"{len(node.children)}"

    # Remove empty parentheses children
    node.children = [child for child in node.children if child.value != '()']


def save_preorder_traversal(tree):
    return ' '.join(tree.preorder_traversal())


def process_file(file_path):
    with open(file_path, 'r') as file:
        node_str = file.read().strip()

    # 构建树结构
    tree = TreeNode('')
    stack = [tree]
    current_node = tree
    i = 0
    while i < len(node_str):
        if node_str[i] == '(':
            new_node = TreeNode('')
            current_node.add_child(new_node)
            stack.append(new_node)
            current_node = new_node
        elif node_str[i] == ')':
            stack.pop()
            current_node = stack[-1]
        else:
            start = i
            while i < len(node_str) and node_str[i] != '(' and node_str[i] != ')':
                i += 1
            value = node_str[start:i]
            current_node.value = value
            i -= 1
        i += 1

    # 健全树结构并保存前序遍历序列
    complete_tree(tree)
    preorder_sequence = save_preorder_traversal(tree)
    return preorder_sequence


input_folder = '/home/rose/code/similar/0623data/testsimpleast' 
output_folder = '/home/rose/code/similar/0623data/testsimpleast/output'  

if not os.path.exists(output_folder):
    os.makedirs(output_folder)

for file_name in os.listdir(input_folder):
    if file_name.endswith('.ast'):
        file_path = os.path.join(input_folder, file_name)
        output_file_name = file_name.replace('.ast', '.json')
        output_file_path = os.path.join(output_folder, output_file_name)

        preorder_sequence = process_file(file_path)

        with open(output_file_path, 'w') as output_file:
            json.dump(preorder_sequence, output_file)

