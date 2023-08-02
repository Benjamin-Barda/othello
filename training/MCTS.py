import torch 
import othello 

import math

class Node : 

    def __init__(
            self, 
            game: othello.Othello,
            args,
            parent = None, 
            prior = 0, 
            action_taken=None): 


        self.game = game 
        self.args = args 
        self.parent = parent 
        self.children = list()

        self.visit_count = 0
        self.value_sum = 0

    def is_leaf(self) : 
        return len(self.children) == 0 

    def select(self) : 
        best_child, best_ucb = None, -np.inf

        for child in self.childer: 
            ucb = self.ucb(child)
            if ucb > best_ucb: 
                best_child = child 
                best_ucb = ucb 
        return best_child

    def expand(self, policy): 
        for action, prob in enumerate(policy): 
            if prob > 0 : 
                self.game.make_move(action)
                child = Node(self.game, self.args, self, prob, action)
                self.children.append(child)


    def ucb(self, child : Node) : 
        '''
        Return the Upper Confidence Bound
        The Q-value is normalized between 0 and 1
        We take 1 - Q-value becauese it is taken from the child perspective which 
        is Another player in Othello.
        '''
        if child.visit_count == 0 : 
            q_value = 0 
        else :  
            # Likelihood of winning at given node 
            # we do '1-' because it is the q_value is taken from the child perspective which is in othello a diffrent player.
            q_value = 1 - ((child.value_sum / child.visit_count) + 1) / 2
        return q_value + self.args["C"] * match.sqrt(math.log(self.visit_count) / child.visit_count) 

    def backpropagate(self, value : Node): 
        self.value_sum += value
        self.visit_count += 1 

        if self.parent is not None : 
            self.parent.backpropagate(-value)


class MCTS: 
    def __init__(self, game : othello.Othello, args, model):
        self.game = game 
        self.args = args
        self.model = model 

    def search(self): 
        root = Node(self.game, self.args)

        for search in range(self.args['num_searches']) : 
            node = root 

            while not node.is_leaf() : 
                node = node.select()

            if not self.game.is_game_ended() : 
                wstate = torch.as_tensor(self.game.get_white_bb()) 
                bstate = torch.as_tensor(self.game.get_black_bb()) 
                
                state = torch.stack(wstate, bstate)
                policy, value = self.model(state).cpu()

                valid_moves = self.game.get_legal_moves()
                policy &= valid_moves

                policy = torch.softmax(policy, axis=1) / torch.sum(policy)

                value = value.item()

                node.expand(policy)

            node.backpropagate(value)

        action_probs = np.zeros(64) 
        for child in root.children : 
            action_probs[child.action_taken] = child.visit_count
        action_probs /= np.sum(action_probs)
        return action_probs


class AlphaSchifo: 
    def __init__(self, model, args, game, optimizer) : 
        self.model = model 
        self.args = args 
        self.game = game 
        self.optimizer = optimizer 
    
    def selfplay(self): 
        ... 

    def train(self): 
        ...
