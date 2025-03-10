# {{ component.identifier.name }} (component code: {{ component.identifier.code }})

{% if component.description %}
## Description 

{{ component.description }}
{% endif %}

{% for error in errors | filter(attribute="component", value=component.identifier.name) | sort(attribute="code") %}

- [`{{error.identifier}} {{ error.name }}`]({{error.name}}.md)
{% if error.documentation.short_description %}
     {{ error.documentation.short_description }}
{% endif %}
   - Message: `{{ error.identifier }} {{ error.message }}`

{% if error.fields | length > 0 %}
   - Fields:
{% for field in error.fields %}
      - `{{ field.name }} : {{ field.type }}`
{% endfor %}
{% endif %}


{% endfor %}  
