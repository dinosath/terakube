// TemplateRenderer.jsx
import React, { useState } from 'react';
import axios from 'axios';
import './TemplateRenderer.css'; // Import the CSS file for styling

const TemplateRenderer = () => {
    const [template, setTemplate] = useState('');
    const [fields, setFields] = useState([{ key: '', values: [''] }]);
    const [renderedTemplate, setRenderedTemplate] = useState('');

    const handleTemplateChange = (event) => {
        setTemplate(event.target.value);
    };

    const handleKeyChange = (index, event) => {
        const newFields = [...fields];
        newFields[index].key = event.target.value;
        setFields(newFields);
    };

    const handleValueChange = (fieldIndex, valueIndex, event) => {
        const newFields = [...fields];
        newFields[fieldIndex].values[valueIndex] = event.target.value;
        setFields(newFields);
    };

    const addField = () => {
        setFields([...fields, { key: '', values: [''] }]);
    };

    const addValue = (index) => {
        const newFields = [...fields];
        newFields[index].values.push('');
        setFields(newFields);
    };

    const removeField = (index) => {
        const newFields = fields.filter((_, i) => i !== index);
        setFields(newFields);
    };

    const removeValue = (fieldIndex, valueIndex) => {
        const newFields = [...fields];
        newFields[fieldIndex].values.splice(valueIndex, 1);
        setFields(newFields);
    };

    const renderTemplate = () => {
        const data = fields.reduce((obj, field) => {
            if (field.key) {
                obj[field.key] = field.values.length > 1 ? field.values : field.values[0];
            }
            return obj;
        }, {});

        axios.post('http://localhost:3000/api/templates/render', {
            data,
            template
        })
            .then(response => {
                const renderedResponse = JSON.stringify(response.data, null, 2);
                setRenderedTemplate(renderedResponse);
            })
            .catch(error => {
                console.error('Error rendering template:', error);
            });
    };

    const createJob = () => {
        const data = fields.reduce((obj, field) => {
            if (field.key) {
                obj[field.key] = field.values.length > 1 ? field.values : field.values[0];
            }
            return obj;
        }, {});

        axios.post('http://localhost:3000/api/templates/createjob', {
            data,
            template
        })
            .then(response => {
                alert('Job created successfully!');
            })
            .catch(error => {
                console.error('Error creating job:', error);
            });
    };

    return (
        <div className="template-renderer-container">
            <div className="input-panel">
                <h3>Data:</h3>
                {fields.map((field, fieldIndex) => (
                    <div key={fieldIndex} className="field-group">
                        <input
                            type="text"
                            value={field.key}
                            onChange={(event) => handleKeyChange(fieldIndex, event)}
                            placeholder="Field name"
                        />
                        {field.values.map((value, valueIndex) => (
                            <div key={valueIndex} className="value-pair">
                                <input
                                    type="text"
                                    value={value}
                                    onChange={(event) => handleValueChange(fieldIndex, valueIndex, event)}
                                    placeholder="Field value"
                                />
                                {field.values.length > 1 && (
                                    <button type="button" onClick={() => removeValue(fieldIndex, valueIndex)}>
                                        Remove Value
                                    </button>
                                )}
                            </div>
                        ))}
                        <button type="button" onClick={() => addValue(fieldIndex)}>
                            Add Value
                        </button>
                        <button type="button" onClick={() => removeField(fieldIndex)}>
                            Remove Field
                        </button>
                    </div>
                ))}
                <button type="button" onClick={addField}>
                    Add Field
                </button>
                <h3>Template:</h3>
                <textarea
                    value={template}
                    onChange={handleTemplateChange}
                    onBlur={renderTemplate} // Render the template when the textarea loses focus
                    placeholder="Enter your template here..."
                    rows="10"
                    cols="50"
                />
                <button onClick={createJob}>Send</button>
            </div>
            <div className="output-panel">
                <h3>Rendered Template:</h3>
                <pre>{renderedTemplate}</pre>
            </div>
        </div>
    );
};

export default TemplateRenderer;