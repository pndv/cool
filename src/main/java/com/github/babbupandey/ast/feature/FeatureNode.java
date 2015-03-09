package com.github.babbupandey.ast.feature;

public abstract class FeatureNode {
    public enum FeatureNodeType {
        ATTRIBUTE,
        METHOD
    }

    public FeatureNodeType type;

    public FeatureNode(FeatureNodeType type) {
        this.type = type;
    }
}
