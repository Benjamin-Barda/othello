import torch
import torch.nn as nn

class SampleModel(nn.Module): 
    '''
    Place Holder model Just for the moment
    '''

    def __init__(self): 
        super().__init__()
        self.backbone = nn.Sequential(
                nn.Conv2d(2, 6, 4), 
                nn.ReLU(inplace=True), 
                nn.Conv2d(6, 12, 4), 
                nn.ReLU(inplace=True),
                nn.Flatten()
        )

        self.eval_head = nn.Linear(48, 1, bias=True)
        self.policy_head = nn.Linear(48, 64, bias=True)

    def forward(self, x) : 

        x = self.backbone(x) 

        eval = self.eval_head(x) 
        pol = self.policy_head(x)

        return (eval, pol)
