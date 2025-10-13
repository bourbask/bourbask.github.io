export const content = {
  tldr: "Complete Ansible playbook for OVH VPS: security hardening, Docker stack, Grafana monitoring, automated deployments. 1 command = production-ready server.",
  sections: [
    {
      id: "introduction",
      title: "Why Ansible for my VPS?",
      content: `# Why Ansible for my VPS?

Managing a server manually is tedious and error-prone. After redoing my OVH VPS config 3 times following "oops" moments, I decided to automate.

## 🚨 Problems Without Automation

**Manual configuration:**
- **Inconsistency** between environments
- **Forgetting** critical config
- **Time wasted** redoing the same actions
- **Human errors** (typos, wrong paths...)

## 💡 Infrastructure as Code Approach

With Ansible, everything becomes:
- **Reproducible**: same result every time
- **Versionable**: Git tracks all changes
- **Testable**: validation before prod
- **Documented**: code is the documentation`,
    },
  ],
};
