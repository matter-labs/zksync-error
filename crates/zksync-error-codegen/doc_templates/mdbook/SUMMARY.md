# Summary

[Introduction](README.md)

# Errors reference

{% for domain in domains %}
- [{{domain.name}}](domains/{{domain.name}}/README.md)
{% for component in components | filter(attribute="domain_name", value=domain.name) %}
    - [{{component.name}}](domains/{{domain.name}}/{{component.name}}/README.md)
{% for error in errors | filter(attribute="domain", value=domain.name) | filter(attribute="component", value=component.name) | sort(attribute="code") %}
        - [{{error.identifier }} {{ error.name }}](domains/{{domain.name}}/{{component.name}}/{{error.name}}.md)
{% endfor %}
{% endfor %}
{% endfor %}
