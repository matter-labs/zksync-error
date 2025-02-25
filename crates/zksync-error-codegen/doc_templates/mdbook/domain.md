# {{ domain.identifier.name }} (domain code: {{ domain.identifier.code }})

{{ domain.description }}


# Components

{% for component in components  | filter(attribute="domain_name", value=domain.identifier.name) %}

## [{{ component.identifier.name }} (code {{ component.identifier.code }})](components/{{component.identifier}}/{{component.identifier.name}}.md)

### Description 

{{ component.description }}

### Errors

{% for error in errors | filter(attribute="component", value=component.identifier.name) | filter(attribute="domain", value=domain.identifier.name) | sort(attribute="code") %}
- [`{{error.identifier}} {{ error.name }}`]({{component.identifier.name}}/{{error.name}}.md)
{% endfor %}

{% endfor %}
