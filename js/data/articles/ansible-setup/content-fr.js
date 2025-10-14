export const content = {
  tldr: "Playbook Ansible complet pour VPS OVH : hardening sécurité, stack Docker, monitoring Grafana, déploiements automatisés. 1 commande = serveur production-ready.",
  sections: [
    {
      id: "introduction",
      title: "Pourquoi Ansible pour mon VPS ?",
      content: `# Pourquoi Ansible pour mon VPS ?

Gérer un serveur à la main, c'est fastidieux et source d'erreurs. Après avoir refait 3 fois la config de mon VPS OVH suite à des "oops", j'ai décidé d'automatiser.

## 🚨 Les Problèmes Sans Automation

**Configuration manuelle :**
- **Incohérence** entre environnements
- **Oublis** de config critique
- **Temps perdu** à refaire les mêmes actions
- **Erreurs humaines** (typos, mauvais chemins...)

## 💡 L'Approche Infrastructure as Code

Avec Ansible, tout devient :
- **Reproductible** : même résultat à chaque fois
- **Versionnable** : Git track tous les changements
- **Testable** : validation avant prod
- **Documenté** : le code est la documentation`,
    },
  ],
};
