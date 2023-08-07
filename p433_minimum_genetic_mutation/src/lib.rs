struct Solution;
use std::collections::{HashSet, VecDeque};
type Gene = Vec<char>;
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let conv_str = |s: String| s.chars().collect::<Gene>();
        let (start_gene, end_gene) = (conv_str(start_gene), conv_str(end_gene));
        let all_letters = HashSet::from(['A','C','G','T']);
        let bank: HashSet<Gene> = bank.into_iter().map(conv_str).into_iter().collect();
        let get_gene_mutations = |gene: &Gene| -> Vec<Gene>{
            let mut mutations = vec![];

            for (i,l) in gene.iter().enumerate() {
                let mut l_variations = all_letters.clone();
                l_variations.remove(l);
                for &nl in l_variations.iter() {
                    let mut new_gene = gene.clone();
                    new_gene[i] = nl;
                    if bank.contains(&new_gene) {
                        mutations.push(new_gene);
                    }
                }
            }

            mutations
        };

        let mut steps = 0;
        let mut queue = VecDeque::from([start_gene.clone()]);
        let mut seen: HashSet<Gene> = HashSet::from([start_gene]);

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let gene = queue.pop_front().unwrap();
                if gene == end_gene {
                    return steps;
                }
                for mutation in get_gene_mutations(&gene) {
                    if seen.insert(mutation.clone()) {
                        queue.push_back(mutation);
                    }
                }
            }
            steps += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AACCGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string()];
        let result = Solution::min_mutation(start_gene, end_gene, bank);
        assert_eq!(result, 1);
    }

    #[test]
    fn t2() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AAACGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string(),"AACCGCTA".to_string(),"AAACGGTA".to_string()];
        let result = Solution::min_mutation(start_gene, end_gene, bank);
        assert_eq!(result, 2);
    }

}
