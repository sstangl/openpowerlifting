## Ansible Deployment

[Ansible](https://www.ansible.com) is an agent-less server deployment and configuration automation framework from [Red Hat](https://www.redhat.com). "Agent-less" means that the servers being configured don't need to run any background task -- they can be configured entirely over SSH.

### Setting up production servers

Assuming you have valid SSH keys, run the following command:

```
ansible-playbook -i production site.yml
```

### Documentation

The [Working with Playbooks](https://docs.ansible.com/ansible/latest/user_guide/playbooks.html#working-with-playbooks) section of the Ansible documentation is particularly helpful. Ansible also has an [example playbook](https://github.com/ansible/ansible-examples) repository.
