package com.github.babbupandey.ast;

import com.github.babbupandey.ast.feature.FeatureNode;
import com.github.babbupandey.exceptions.ClassTypeNotDefinedException;

import java.util.List;

public class ClassNode {
    private TypeNode classType;
    private TypeNode parentType;
    private List<FeatureNode> featureList;

    public ClassNode(TypeNode classType, TypeNode parentType, List<FeatureNode> featureList) {
        if(classType == null) {
            throw new ClassTypeNotDefinedException();
        }
        this.classType = classType;

        if (parentType != null) {
            //todo: check parent type exists
            this.parentType = parentType;
        }

        this.featureList = featureList;
    }

}
