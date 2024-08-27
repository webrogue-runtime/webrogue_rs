package io.github.webrogue_runtime;

import android.graphics.Color;
import android.os.Bundle;
import android.view.ViewGroup;
import android.widget.RelativeLayout;
import android.widget.TextView;

import androidx.annotation.Keep;

import org.libsdl.app.SDLActivity;

import java.nio.charset.StandardCharsets;

public class WebrogueActivity extends SDLActivity {
    private TextView textView;
    private String consoleText = "";
    private static WebrogueActivity sharedWebrogueActivity;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        sharedWebrogueActivity = this;
        super.onCreate(savedInstanceState);

        setWindowStyle(true);

        RelativeLayout.LayoutParams layoutParams = new RelativeLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        );

        textView = new TextView(this);
        textView.setTextColor(Color.parseColor("#ffd9ff04"));
        layoutParams.addRule(RelativeLayout.ALIGN_TOP);
        mLayout.addView(textView, layoutParams);
    }


    @Keep
    public static void printBytes(byte[] bytes) {
        sharedWebrogueActivity.runOnUiThread(() -> {
            sharedWebrogueActivity.consoleText += new String(bytes, StandardCharsets.UTF_8) + "\n";
            sharedWebrogueActivity.textView.setText(sharedWebrogueActivity.consoleText);
        });
    }
    @Override
    protected String[] getLibraries() {
        return new String[]{ "webrogue" };
    }
}
