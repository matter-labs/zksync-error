# Summary

[Introduction](README.md)

# Errors reference

{% for domain in domains %}
- [{{domain.identifier.name}}](domains/{{domain.identifier.name}}/README.md)

    {% for component in components | filter(attribute="domain_name", value=domain.identifier.name) %}
    
    - [{{component.identifier.name}}](domains/{{domain.identifier.name}}/{{component.identifier.name}}/README.md)

        {% for error in errors | filter(attribute="domain", value=domain.identifier.name) | filter(attribute="component", value=component.identifier.name) | sort(attribute="code") %}

        - [{{error.identifier }} {{ error.name }}](domains/{{domain.identifier.name}}/{{component.identifier.name}}/{{error.name}}.md)

        {% endfor %}
        
    {% endfor %}
    
{% endfor %}
