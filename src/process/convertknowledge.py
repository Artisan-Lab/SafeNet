import os
import json

def process_files(folder_path):
    data = []
    for filename in os.listdir(folder_path):
        if filename.endswith('.json'):
            file_path = os.path.join(folder_path, filename)
            with open(file_path, 'r') as file:
                content = file.read()
            
            # Extract required information
            id = filename[:-5]
            category, _, index = id.rpartition('_')
            
            # Create JSON object
            json_obj = {
                "id": id,
                "category": category,
                "text": content,
                "index": int(index)
            }
            
            data.append(json_obj)
    
    # Write data to output JSON file
    output_path = os.path.join(folder_path, 'knowledgebase.json')
    with open(output_path, 'w') as output_file:
        json.dump(data, output_file, indent=4)
    
    print("Output JSON file generated successfully.")

# Provide the folder path as an argument
folder_path = '/home/rose/code/similar/0623data/testtree0628'
process_files(folder_path)
