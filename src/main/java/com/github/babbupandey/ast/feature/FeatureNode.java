package com.github.babbupandey.ast.feature;

public abstract class FeatureNode {
    public FeatureNodeType type;

    public FeatureNode(FeatureNodeType type) {
        this.type = type;
    }

    public enum FeatureNodeType {
        ATTRIBUTE,
        METHOD
    }
}
