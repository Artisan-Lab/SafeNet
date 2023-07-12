import re
import os
from collections import deque

input_dir_path = '/home/rose/code/similar/0623data/testast0628'
output_dir_path = '/home/rose/code/similar/0623data/testsimpleast'
output_folder = '/home/rose/code/similar/0623data/testtree0628'


class Node:
    def __init__(self, val=0, next=None, father=None, num_branches=0):
        self.val = val
        self.next = next
        self.father = father
        self.num_branches = num_branches


def get_str(file):
    with open(file, 'r') as f:
        return f.readline()


def get_tree(s):
    node = Node()
    node_list, edge_list = [], [[], []]
    num_node = 1
    node_name = ''
    is_leaf = True  # 初始值为True
    for i in s:
        if i == '(':
            node_list.append(node.num_branches)
            node.next = Node(node.num_branches, father=node, num_branches=node.num_branches + 1)
            node = node.next
            num_node += 1
            if node.father.val != 0:
                edge_list[0].append(node.father.val)
                edge_list[1].append(node.val)
            is_leaf = True  # 读取到'('时，将is_leaf设为False
        elif i == ')':
            if node_name != '':
                node_list.append(node_name)
                node.next = Node(node_name, father=node, num_branches=node.num_branches + 1)
                edge_list[0].append(node.val)
                edge_list[1].append(node_name)
                node_name = ''
            node = node.father
            is_leaf = False  # 读取到')'时，将is_leaf设为True
        else:
            node_name += i
    return node_list, edge_list


def write_tree(file, node_list, edge_list):
    output_file = os.path.join(output_folder, file)
    with open(output_file, 'w') as f:
        f.write(' '.join([str(n) for n in node_list]) + '\n')
        f.write(' '.join([str(n) for n in edge_list[0]]) + '\n')
        f.write(' '.join([str(n) for n in edge_list[1]]) + '\n')


def get_level_order(node_list, edge_list):
    root = node_list[0]
    queue = deque()
    queue.append(root)
    level_order = []
    while queue:
        node_val = queue.popleft()
        level_order.append(str(node_val))
        children = [edge_list[1][i] for i, val in enumerate(edge_list[0]) if val == node_val]
        queue.extend(children)
    return level_order


def simplify_bracket(src):
    stack = []
    splited = re.split(r"([\(\)])", src)
    for p in [e for e in splited if e]:
        if p != ")":
            stack.append(p)
        else:
            temp = ""
            for i, _ in enumerate(range(len(stack))):
                s_pop = stack.pop(-1)
                if s_pop == "(":
                    if i <= 1:
                        if "(" not in temp:
                            stack.append(f"({temp})")
                        else:
                            stack.append(temp)
                    else:
                        stack.append(f'({temp})')

                    break
                else:
                    temp = s_pop + temp
    return "".join(stack)


for input_file in os.listdir(input_dir_path):
    var_dict = {}
    res_dict = {
        "unsafe": None,
        "Box": None,
        "MaybeUninit": None,
    }

    if input_file.endswith('.ast'):
        with open(os.path.join(input_dir_path, input_file), 'r') as f1, open(
                os.path.join(output_dir_path, 'simplify_tree_' + input_file), 'w') as f2:
            string = f1.read()
            string = re.sub('ident:[ \w]{1,}\([\n ]*([\w]{1,},)[\n ]*\),', 'ident: \g<1>', string, flags=re.S)
            string = re.sub('method:[ \w]{1,}\([\n ]*([\w]{1,},)[\n ]*\),', 'ident: \g<1>', string, flags=re.S)
            string = string.replace('\n', '').replace(' ', '')
            string = string.replace("{", "(").replace("}", ")").replace("[", "(").replace("]", ")")
            string = string.replace('(eq,', '( eq,').replace('(Mut,', '( Mut,').replace('(Dot2,', '( Dot2,').replace(
                '(Colon2,', '( Colon2,')
            result = re.findall(r'([()])|(ident:\w*)|(\w*token\w*:([^N]\w*))|( eq,)|( Mut,)|( Colon2,)', string,
                                flags=re.I)
            result = [''.join(i).strip().strip(',') for i in result]
            result = [i if i in '()' else f'({i})' for i in result]
            output = ''.join(result)
            result = re.findall('\((ident:\w{1,})\)', output)
            filter_res = [i.split(':') for i in result if i.split(':')[-1] not in res_dict]
            di = {}
            for i in filter_res:
                if i[-1] not in di:
                    di[i[-1]] = 'id' + str(len(di) + 1)
                    res_dict[i[-1]] = None
            for k in di:
                output = output.replace('ident:' + k, 'ident:' + di[k])

            token_find = re.findall(r'(([^\(\):]*token[^\(\):]*):([^\(\):]+))', output)
            for t, p, s in token_find:
                output = output.replace(t, p)
            ident_find = re.findall(r'(([^\(\):]*ident[^\(\):]*):([^\(\):]+))', output)
            for t, p, s in ident_find:
                output = output.replace(t, s)
            output = output.replace("()", "")
            simplify_tree = simplify_bracket(output)
            print("simplify_tree: ", simplify_tree)
            f2.write(simplify_tree)

            node_list, edge_list = get_tree(simplify_tree)
            output_file = os.path.join(output_folder, f'{input_file[:-4]}.json')
            with open(output_file, 'w') as f:
                f.write(str(node_list))
            pattern = r'\.json$'
            for filename in os.listdir(output_folder):
                if re.search(pattern, filename):
                    file_path = os.path.join(output_folder, filename)
                    with open(file_path, 'r') as file:
                        content = file.read()
                        new_content = content.replace("[", '').replace("]", '').replace(",", '').replace("'", '').replace("_token", '')
                        new_content = re.sub(r'[":{}\[\]]', '', new_content)
                        with open(file_path, 'w') as file:
                            file.write(new_content)
            for filename in os.listdir(output_folder):
                if filename.endswith('.json'):
                    new_filename = filename.replace('_root_rustsrc_0623', '')
                    os.rename(os.path.join(output_folder, filename), os.path.join(output_folder, new_filename))




